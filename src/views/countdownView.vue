<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface CountdownItem {
  title: string
  date: string
  remaining: {
    days: number
    hours: number
    minutes: number
    seconds: number
  }
}

const countdowns = ref<CountdownItem[]>([])
const error = ref<string | null>(null)
let intervalId: number | null = null

const calculateTimeRemaining = (targetDate: string) => {
  const target = new Date(targetDate).getTime()
  const now = new Date().getTime()
  const difference = target - now

  if (difference < 0) {
    return { days: 0, hours: 0, minutes: 0, seconds: 0 }
  }

  return {
    days: Math.floor(difference / (1000 * 60 * 60 * 24)),
    hours: Math.floor((difference % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60)),
    minutes: Math.floor((difference % (1000 * 60 * 60)) / (1000 * 60)),
    seconds: Math.floor((difference % (1000 * 60)) / 1000)
  }
}

const updateCountdowns = () => {
  countdowns.value = countdowns.value.map(countdown => ({
    ...countdown,
    remaining: calculateTimeRemaining(countdown.date)
  }))
}

const fetchCountdowns = async () => {
  try {
    const response = await fetch('https://wiki.alexclimie.com/countdowns')
    const data = await response.json()
    
    countdowns.value = data.map(([title, date]: [string, string]) => ({
      title,
      date,
      remaining: calculateTimeRemaining(date)
    }))
    
    if (countdowns.value.length > 0 && !intervalId) {
      intervalId = setInterval(updateCountdowns, 1000)
    }
  } catch (e) {
    error.value = 'Failed to load countdowns. Please try again later.'
    console.error('Error fetching countdowns:', e)
  }
}

onMounted(() => {
  fetchCountdowns()
})

onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId)
  }
})
</script>

<template>
  <div class="container">
    <h1>Countdown Timers</h1>

    <div v-if="error" class="error">
      {{ error }}
    </div>

    <div v-else-if="countdowns.length === 0" class="loading">
      Loading countdowns...
    </div>

    <div v-else class="countdown-grid">
      <div v-for="countdown in countdowns" :key="countdown.title" class="countdown-card">
        <h2>{{ countdown.title }}</h2>
        
        <div class="time-grid">
          <div class="time-block">
            <div class="time-value">{{ countdown.remaining.days }}</div>
            <div class="time-label">Days</div>
          </div>
          
          <div class="time-block">
            <div class="time-value">{{ countdown.remaining.hours }}</div>
            <div class="time-label">Hours</div>
          </div>
          
          <div class="time-block">
            <div class="time-value">{{ countdown.remaining.minutes }}</div>
            <div class="time-label">Minutes</div>
          </div>
          
          <div class="time-block">
            <div class="time-value">{{ countdown.remaining.seconds }}</div>
            <div class="time-label">Seconds</div>
          </div>
        </div>

        <div class="target-date">
          Target: {{ new Date(countdown.date).toLocaleString() }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
  font-family: Arial, sans-serif;
}

h1 {
  text-align: center;
  color: #333;
  margin-bottom: 30px;
}

.error {
  background-color: #ffebee;
  color: #c62828;
  padding: 15px;
  border-radius: 4px;
  text-align: center;
  margin-bottom: 20px;
}

.loading {
  text-align: center;
  color: #666;
}

.countdown-grid {
  display: grid;
  gap: 20px;
}

.countdown-card {
  background-color: #fff;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: box-shadow 0.3s ease;
}

.countdown-card:hover {
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.countdown-card h2 {
  margin: 0 0 20px 0;
  color: #333;
  font-size: 1.5em;
}

.time-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
  margin-bottom: 15px;
}

.time-block {
  background-color: #f5f5f5;
  border-radius: 4px;
  padding: 10px;
  text-align: center;
}

.time-value {
  font-size: 1.8em;
  font-weight: bold;
  color: #333;
  margin-bottom: 5px;
}

.time-label {
  font-size: 0.9em;
  color: #666;
}

.target-date {
  text-align: center;
  color: #666;
  font-size: 0.9em;
  margin-top: 15px;
}

/* Responsive design */
@media (max-width: 600px) {
  .container {
    padding: 10px;
  }

  .time-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
  }

  .time-value {
    font-size: 1.5em;
  }

  .time-label {
    font-size: 0.8em;
  }
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  body {
    background-color: #1a1a1a;
  }

  .countdown-card {
    background-color: #2d2d2d;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .countdown-card:hover {
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
  }

  h1, .countdown-card h2, .time-value {
    color: #fff;
  }

  .time-block {
    background-color: #3d3d3d;
  }

  .time-label, .target-date {
    color: #aaa;
  }

  .loading {
    color: #aaa;
  }
}
</style>