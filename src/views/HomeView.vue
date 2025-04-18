<script setup lang="ts">
import { ref, computed } from 'vue'
import axios from 'axios'
import { SearchIcon, LoaderIcon, GraduationCapIcon } from 'lucide-vue-next'
import { API_BASE_URL } from '@/constants'

const searchQuery = ref('')
const selectedId = ref('')
const searchResults = ref<{id: string, name: string}[]>([])
const classData = ref([])
const errorMessage = ref('')
const isLoading = ref(false)
const showSuggestions = ref(false)
const sharedClassesData = ref<[{id: string, name: string}, number][]>([])

type SharedClassData = [string, number]; // Tuple type: [name, count]

const searchNames = async () => {
  if (searchQuery.value.length < 2) {
    searchResults.value = []
    return
  }
  
  try {
    const response = await axios.get(API_BASE_URL + `/prefix/${searchQuery.value}`)
    searchResults.value = response.data
  } catch (error) {
    console.error('Failed to fetch suggestions:', error)
    searchResults.value = []
  }
}

const selectStudent = (name: any, id: any) => {
  searchQuery.value = name
  selectedId.value = id
  localStorage.setItem("selected", id)
  localStorage.setItem("name", name)
  showSuggestions.value = false
  fetchData()
}

const fetchData = async () => {
  if (!selectedId.value) return
  
  errorMessage.value = ''
  isLoading.value = true
  showSuggestions.value = false

  try {
    let key: string | null = ""
    if (localStorage.getItem("session")) {
      key = localStorage.getItem("session")
    }
    else {
      key = (Math.random() + 1).toString(36).substring(7);
      localStorage.setItem("session", key)
    }
    const response = await axios.get(API_BASE_URL + `/get_data/${selectedId.value}/${key}`)
    classData.value = response.data[0]

    sharedClassesData.value = response.data[1]
    sharedClassesData.value.sort(function(a, b) {
      return b[1] - a[1];
    });
    sharedClassesData.value = sharedClassesData.value.slice(0, 15); 

  } catch (error) {
    errorMessage.value = 'Failed to fetch data. Please try again.'
    console.error(error)
  } finally {
    isLoading.value = false
  }
}

const totalClasses = computed(() => {
  return classData.value.reduce((total, item) => total + parseInt(item[1]), 0)
})

const getProgressColor = (classesLeft: number) => {
  if (classesLeft <= 2) return 'progress-success'
  if (classesLeft <= 4) return 'progress-warning'
  return 'progress-default'
}

if (localStorage.getItem("selected") && localStorage.getItem("name")) {
  selectStudent(localStorage.getItem("name"), localStorage.getItem("selected"));
}
</script>

<template>
  <main class="main-container">
    <div class="content-wrapper">
      <!-- Header Section -->
      <div class="header">
        <h1 class="title">How much time is left?</h1>
        <p class="classes-label">As no more classes are left, names have now anonymized</p>
      </div>

      <!-- Search Section -->
      <div class="search-container">
        <div class="search-wrapper">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search by name..."
            class="search-input"
            @input="searchNames"
            @focus="showSuggestions = true"
          />
          <SearchIcon class="search-icon" />
          
          <!-- Search Suggestions -->
          <div v-if="showSuggestions && searchResults.length > 0" class="search-suggestions">
            <div
              v-for="student in searchResults"
              :key="student.id"
              class="suggestion-item"
              @click="selectStudent(student.name, student.id)"
            >
              <span class="suggestion-name">{{ student.name }}</span>
              <span class="suggestion-id">ID: {{ student.id }}</span>
            </div>
          </div>
        </div>
        <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>
      </div>

      <!-- Loading State -->
      <div v-if="isLoading" class="loading-state">
        <LoaderIcon class="loading-icon" />
      </div>

      <!-- Data Display Section -->
      <div v-else-if="classData.length > 0" class="data-container">
        <!-- Summary Card -->
        <div class="summary-card">
          <div class="summary-content">
            <div class="summary-text">
              
              <div class="summary-numbers">
                <p class="total-classes">{{ totalClasses }}</p>
                <p class="classes-label">classes remaining</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Classes Grid -->
        <div class="classes-grid">
          <div
            v-for="(item, index) in classData"
            :key="index"
            class="class-card"
            :style="'background: linear-gradient(to right, #f9f9f9 ' + item[1] * 10 +'% ,#f9f9f9 25%, #f9f9f9 25%,white 25%,white 100%)'"
          >
            <div class="class-header">
              <div class="class-info">
                <h3 class="class-title">{{ item[0] }}</h3>
              </div>
            </div>
            
            <div class="progress-container">
              <span :class="['remaining-number', getProgressColor(item[1])]">
                {{ item[1] }} <span v-if="item[1]!='1'">classes</span><span v-else>class</span> left
              </span>
            </div>
          </div>
        </div>

        <!-- Shared Classes Visualization -->
        <div v-if="sharedClassesData.length > 0" class="shared-classes-container">
          <h2 class="shared-classes-title">Who will you be spending that time with?</h2>
          <div class="shared-classes-grid">
            <button
              v-for="[student, count] in sharedClassesData"
              :key="student.id"
              class="shared-class-bar"
              :style="{ width: `${Math.max((count / Math.max(...sharedClassesData.map(([_, c]) => c))) * 85, 20)}%` }"
              @click="selectStudent(student.name, student.id)"
            >
              <span class="shared-class-name">{{ student.name }}</span>
              <span class="shared-class-count">{{ count }} classes</span>
            </button>
          </div>
        </div>
      </div>

      <!-- No Data State -->
      <div v-else-if="!isLoading && classData.length === 0 && !errorMessage" class="empty-state">
        <GraduationCapIcon class="empty-icon" />
        <p class="empty-text">Enter your name to view your graduation progress</p>
        <p class = "kevingong">Kwcin Gong was here</p>
      </div>
    </div>
  </main>
</template>

<style scoped>
.kevingong {
  color: rgb(254, 254, 254);
}

.main-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
}

.content-wrapper {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem 1rem;
}

.header {
  text-align: center;
  margin-bottom: 3rem;
}

.title {
  font-size: 2.5rem;
  font-weight: bold;
  background: linear-gradient(to right, #7c3aed, #4f46e5);
  -webkit-background-clip: text;
  color: transparent;
  margin-bottom: 1rem;
}

.subtitle {
  color: #666;
  max-width: 600px;
  margin: 0 auto;
}

.search-container {
  max-width: 500px;
  margin: 0 auto 3vw;
}

.search-wrapper {
  position: relative;
  margin: 0 auto;
  display: flex;
  justify-content: center; 
  align-items: center;
}

.search-input {
  width: 100%;
  padding: 0.75rem 1rem 0.75rem 3rem;
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid #e5e7eb;
  border-radius: 0.75rem;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
  margin: 0 auto;
}

.search-input:focus {
  outline: none;
  border-color: #7c3aed;
  box-shadow: 0 0 0 3px rgba(124, 58, 237, 0.1);
}

.search-icon {
  position: absolute;
  right: 1rem;
  font-size: 1.5rem;
  color: #888;
}

.search-button {
  position: absolute;
  right: 0.5rem;
  top: 50%;
  transform: translateY(-50%);
  padding: 0.5rem 1rem;
  background: linear-gradient(to right, #7c3aed, #4f46e5);
  color: white;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.search-button:hover {
  opacity: 0.9;
}

.error-message {
  color: #ef4444;
  text-align: center;
  margin-top: 0.5rem;
}

.loading-state {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 3rem 0;
}

.loading-icon {
  width: 2rem;
  height: 2rem;
  color: #7c3aed;
  animation: spin 1s linear infinite;
}

.data-container {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.summary-card {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 1rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  max-width: 600px;
  margin: 0 auto;
  min-width: 23vw;
}

.summary-content {
  display: flex;
  align-items: center;
}

.summary-text {
  display: flex;
  flex-direction: column;
  margin: 0 auto;
}

.summary-label {
  color: #666;
}

.summary-numbers {
  display: flex;
  align-items: baseline;
  gap: 0.5rem;
}

.total-classes {
  font-size: 2rem;
  font-weight: bold;
  color: #7c3aed;
}

.classes-label {
  color: #666;
}

.progress-circle-container {
  position: relative;
  width: 6rem;
  height: 6rem;
}

.progress-circle {
  transform: rotate(-90deg);
}

.progress-circle-bg {
  fill: none;
  stroke: #e5e7eb;
  stroke-width: 2.5;
}

.progress-circle-value {
  fill: none;
  stroke: #7c3aed;
  stroke-width: 2.5;
  stroke-dasharray: 100;
  transition: stroke-dashoffset 1s ease-out;
}

.progress-icon {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #7c3aed;
}

.classes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
}

.class-card {
  
  backdrop-filter: blur(10px);
  border-radius: 0.75rem;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
}

.class-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.class-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.class-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.class-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: #1f2937;
}

.class-subtitle {
  font-size: 0.875rem;
  color: #666;
}

.progress-container {
  position: relative;
}

.remaining-number {
  font-size: 1.25rem;
  font-weight: 600;
}

.remaining-number.progress-success { color: #10b981; }
.remaining-number.progress-warning { color: #f59e0b; }
.remaining-number.progress-default { color: #7c3aed; }


.empty-state {
  text-align: center;
  padding: 3rem;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 0.75rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.empty-icon {
  width: 4rem;
  height: 4rem;
  color: #9ca3af;
  margin: 0 auto 1rem;
}

.empty-text {
  color: #666;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@media (max-width: 768px) {
  .content-wrapper {
    padding: 1rem;
  }

  .title {
    font-size: 2rem;
  }

  .classes-grid {
    grid-template-columns: 1fr;
  }

  .summary-card {
    padding: 1.5rem;
  }
}

.search-suggestions {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: white;
  border: 1px solid #e5e7eb;
  border-top: none;
  border-radius: 0 0 0.75rem 0.75rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  max-height: 300px;
  overflow-y: auto;
  z-index: 50;
}

.suggestion-item {
  padding: 0.75rem 1rem;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: background-color 0.2s ease;
}

.suggestion-item:hover {
  background-color: #f3f4f6;
}

.suggestion-name {
  font-weight: 500;
  color: #1f2937;
}

.suggestion-id {
  color: #6b7280;
  font-size: 0.875rem;
}

/* Add smooth scrollbar for suggestions */
.search-suggestions {
  scrollbar-width: thin;
  scrollbar-color: #cbd5e1 transparent;
}

.search-suggestions::-webkit-scrollbar {
  width: 6px;
}

.search-suggestions::-webkit-scrollbar-track {
  background: transparent;
}

.search-suggestions::-webkit-scrollbar-thumb {
  background-color: #cbd5e1;
  border-radius: 3px;
}

/* Adjust search input styles */
.search-input {
  padding-right: 4rem; /* Adjust padding to accommodate the icon */
}

/* Add a subtle border when suggestions are shown */
.search-wrapper:focus-within {
  border-radius: 0.75rem 0.75rem 0 0;
}

.shared-classes-container {
  margin-top: 2rem;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 0.75rem;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.shared-classes-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 1.5rem;
}

.shared-classes-grid {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.shared-class-bar {
  background: linear-gradient(to right, #ff9f43, #ff7f50);
  color: white;
  padding: 1rem;
  border-radius: 0.5rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: width 0.3s ease;
}

.shared-class-name {
  font-weight: 500;
  margin-right: 1rem;
}

.shared-class-count {
  font-weight: 600;
}

@media (max-width: 768px) {
  .shared-classes-container {
    padding: 1rem;
  }
  
  .shared-class-bar {
    width: 100% !important;
  }
}
</style>

