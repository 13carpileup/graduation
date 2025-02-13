<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { API_BASE_URL } from '@/constants'
const idmap = new Map()

const error = ref<string | null>(null)
const searchResults = ref<[[string, string], number][]>([])


const fetchNames = async () => {
  try {
    const response = await fetch(API_BASE_URL + '/get_all_names')
    const data = await response.json()

    for (let i = 0; i < data.length; i++) {
      idmap.set(data[i][1], data[i][0])
    }

    console.log("TEST: 9668 is " + idmap.get("9668"))

  } catch (e) {
    error.value = 'Failed to load countdowns. Please try again later.'
    console.error('Error fetching countdowns:', e)
  }
}

const fetchConnections = async () => {
  try {
    const response = await fetch(API_BASE_URL + '/get_connections')
    const data = await response.json()
    searchResults.value = data

  } catch (e) {
    error.value = 'Failed to load countdowns. Please try again later.'
    console.error('Error fetching countdowns:', e)
  }
}

const initConnections = () => {

}

onMounted(() => {
  fetchNames()
  fetchConnections()
  initConnections()
})
</script>

<template>
  <div class = "app">
    <div v-if="searchResults">
      <div v-for="item in searchResults">
        {{ console.log(item[0][0]) }}
        {{ idmap.get(item[0][0].toString()) }} ({{ item[0][0] }}) - {{ idmap.get(item[0][1].toString()) }} ({{ item[0][1] }}) has value {{ item[1] }}
      </div>
    </div>

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