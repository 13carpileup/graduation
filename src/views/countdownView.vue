<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { API_BASE_URL } from '@/constants'


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
    const response = await fetch(API_BASE_URL + '/countdowns')
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
  <div class="app">
    <div class="container">
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
              <div class="time-label">days</div>
            </div>
            
            <div class="time-block">
              <div class="time-value">{{ countdown.remaining.hours }}</div>
              <div class="time-label">hours</div>
            </div>
            
            <div class="time-block">
              <div class="time-value">{{ countdown.remaining.minutes }}</div>
              <div class="time-label">minutes</div>
            </div>
            
            <div class="time-block">
              <div class="time-value">{{ countdown.remaining.seconds }}</div>
              <div class="time-label">seconds</div>
            </div>
          </div>

          <div class="target-date">
            {{ new Date(countdown.date).toLocaleString() }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
html, body {
  margin: 0;
  padding: 0;
  background-color: #f5f5f5;
  min-height: 100vh;
}
</style>

<style scoped>
.app {
  min-height: 100vh;
  padding: 20px 0;
}

.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 0 20px;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
}

.countdown-grid {
  display: grid;
  gap: 20px;
}

.countdown-card {
  background-color: #ffffff;
  padding: 24px;
  border-radius: 8px;
}

.countdown-card h2 {
  margin: 0 0 24px 0;
  color: #333;
  font-size: 1.4em;
  font-weight: 500;
}

.time-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 20px;
}

.time-block {
  text-align: center;
}

.time-value {
  font-size: 2em;
  font-weight: 500;
  color: #333;
  margin-bottom: 4px;
}

.time-label {
  font-size: 0.85em;
  color: #666;
}

.target-date {
  text-align: right;
  color: #666;
  font-size: 0.9em;
  margin-top: 16px;
}

.error {
  color: #e53935;
  text-align: center;
  padding: 16px;
}

.loading {
  text-align: center;
  color: #666;
}

@media (max-width: 600px) {
  .container {
    padding: 0 16px;
  }

  .countdown-card {
    padding: 16px;
  }

  .time-grid {
    gap: 12px;
  }

  .time-value {
    font-size: 1.6em;
  }
}
</style>