<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const error = ref<string | null>(null)


const fetchCountdowns = async () => {
  try {
    const response = await fetch('https://127.0.0.1/get_connections')
    const data = await response.json()

  } catch (e) {
    error.value = 'Failed to load countdowns. Please try again later.'
    console.error('Error fetching countdowns:', e)
  }
}

onMounted(() => {
  fetchCountdowns()
})
</script>

<template>
  <div class = "app">
    work in progress...
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