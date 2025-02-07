<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <header class="app-header">
    <div class="header-content">
      <nav class="nav-container">
        <RouterLink to="/" class="nav-link">
          Graduation Tracker
        </RouterLink>
        <RouterLink to="/countdown" class="nav-link">
          Countdown
        </RouterLink>
      </nav>
    </div>
  </header>

  <RouterView />
</template>

<style scoped>
.app-header {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid #e5e7eb;
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 1rem;
}

.nav-container {
  display: flex;
  align-items: center;
  height: 4rem;
}

.nav-link {
  margin: 3rem;
  float:left;
  font-size: 1.125rem;
  font-weight: 600;
  background: linear-gradient(to right, #7c3aed, #4f46e5);
  -webkit-background-clip: text;
  color: transparent;
  text-decoration: none;
  transition: opacity 0.3s ease;
}

.nav-link:hover {
  opacity: 0.8;
}
</style>

<style>
body {
  margin: 0;
  min-height: 100vh;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
}

#app {
  min-height: 100vh;
}
</style>

