<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { TimerIcon } from 'lucide-vue-next'

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
    
    // Start the interval only if we have countdowns
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
  <div class="min-h-screen bg-gray-50 py-8 px-4">
    <div class="max-w-3xl mx-auto">
      <h1 class="text-3xl font-bold text-center mb-8 text-gray-800">
        <TimerIcon class="inline-block mr-2 mb-1" />
        Countdown Timers
      </h1>

      <div v-if="error" class="bg-red-50 text-red-600 p-4 rounded-lg mb-4">
        {{ error }}
      </div>

      <div v-else-if="countdowns.length === 0" class="text-center text-gray-500">
        Loading countdowns...
      </div>

      <div v-else class="grid gap-6">
        <div
          v-for="countdown in countdowns"
          :key="countdown.title"
          class="bg-white rounded-xl shadow-sm overflow-hidden hover:shadow-md transition-shadow duration-300"
        >
          <div class="p-6">
            <h2 class="text-xl font-semibold text-gray-800 mb-4">
              {{ countdown.title }}
            </h2>
            
            <div class="grid grid-cols-4 gap-4 text-center">
              <div class="bg-gray-50 rounded-lg p-3">
                <div class="text-2xl font-bold text-gray-800">
                  {{ countdown.remaining.days }}
                </div>
                <div class="text-sm text-gray-500">Days</div>
              </div>
              
              <div class="bg-gray-50 rounded-lg p-3">
                <div class="text-2xl font-bold text-gray-800">
                  {{ countdown.remaining.hours }}
                </div>
                <div class="text-sm text-gray-500">Hours</div>
              </div>
              
              <div class="bg-gray-50 rounded-lg p-3">
                <div class="text-2xl font-bold text-gray-800">
                  {{ countdown.remaining.minutes }}
                </div>
                <div class="text-sm text-gray-500">Minutes</div>
              </div>
              
              <div class="bg-gray-50 rounded-lg p-3">
                <div class="text-2xl font-bold text-gray-800">
                  {{ countdown.remaining.seconds }}
                </div>
                <div class="text-sm text-gray-500">Seconds</div>
              </div>
            </div>

            <div class="mt-4 text-sm text-gray-500 text-center">
              Target: {{ new Date(countdown.date).toLocaleString() }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.countdown-pulse {
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
</style>