<script setup>
import * as d3 from 'd3'
import { ref, onMounted, onUnmounted } from 'vue'
import { API_BASE_URL } from '@/constants'

const nodes = ref([])
const data = ref({})
const error = ref('')
const simulation = ref(null)
const width = ref(800)
const height = ref(600)

const idmap = new Map()
const connectionsData = ref([])

const updateDimensions = () => {
  const container = document.querySelector('.graph-container')
  if (container) {
    width.value = container.clientWidth
    height.value = container.clientHeight
  }
}

const fetchNames = async () => {
  try {
    const response = await fetch(API_BASE_URL + '/get_all_names')
    const data = await response.json()

    for (let i = 0; i < data.length; i++) {
      idmap.set(data[i][1], data[i][0])
      nodes.value.push({
        id: data[i][1],
        name: data[i][0],
      })
    }
    console.log("Names fetched!", nodes.value)
  } catch (e) {
    error.value = 'Failed to load names. Please try again later.'
    console.error('Error fetching names:', e)
  }
}

const initGraph = () => {
  console.log('Initiating data...')
  
  const validNodeIds = new Set(nodes.value.map(node => node.id))
  
  const links = connectionsData.value
    .filter(conn => {
      const sourceExists = validNodeIds.has(conn[0][0])
      const targetExists = validNodeIds.has(conn[0][1])
      
      if (!sourceExists) {
        console.warn(`Source node ${conn[0][0]} not found`)
      }
      if (!targetExists) {
        console.warn(`Target node ${conn[0][1]} not found`)
      }
      
      return sourceExists && targetExists && conn[1] >= 50
    })
    .map(conn => ({
      source: conn[0][0],
      target: conn[0][1],
      value: conn[1] / 10 || 1
    }))

  console.log('Valid links created:', links)

  data.value = { 
    nodes: nodes.value, 
    links: links 
  }

  d3.select('#graph').selectAll('*').remove()

  const svg = d3.select('#graph')
    .attr('width', width.value)
    .attr('height', height.value)

  const g = svg.append('g')
    .attr('class', 'everything')

  const zoom = d3.zoom()
    .scaleExtent([0.1, 4])
    .on('zoom', (event) => {
      g.attr('transform', event.transform)
    })
  
  svg.call(zoom)

  const link = g.append('g')
    .selectAll('line')
    .data(data.value.links)
    .join('line')
    .attr('stroke', '#999')
    .attr('stroke-opacity', 0.6)
    .attr('stroke-width', d => Math.sqrt(d.value))

  const node = g.append('g')
    .selectAll('g')
    .data(data.value.nodes)
    .join('g')
    .call(drag(simulation))

  node.append('circle')
    .attr('r', 8)
    .attr('fill', '#69b3a2')

  node.append('text')
    .text(d => d.name)
    .attr('x', 12)
    .attr('y', 4)
    .style('font-size', '12px')
    .style('fill', '#333')

  if (nodes.value.length > 0) {
    simulation.value = d3.forceSimulation(data.value.nodes)
      .force('link', d3.forceLink(data.value.links)
        .id(d => d.id)
        .distance(100))
      .force('charge', d3.forceManyBody().strength(-200))
      .force('center', d3.forceCenter(width.value / 2, height.value / 2))
      .on('tick', () => {
        link
          .attr('x1', d => d.source.x)
          .attr('y1', d => d.source.y)
          .attr('x2', d => d.target.x)
          .attr('y2', d => d.target.y)

        node
          .attr('transform', d => `translate(${d.x},${d.y})`)
      })
  }
}

const drag = (simulation) => {
  const dragstarted = (event, d) => {
    if (!event.active && simulation.value) simulation.value.alphaTarget(0.3).restart()
    d.fx = d.x
    d.fy = d.y
  }

  const dragged = (event, d) => {
    d.fx = event.x
    d.fy = event.y
  }

  const dragended = (event, d) => {
    if (!event.active && simulation.value) simulation.value.alphaTarget(0)
    d.fx = null
    d.fy = null
  }

  return d3.drag()
    .on('start', dragstarted)
    .on('drag', dragged)
    .on('end', dragended)
}

const fetchConnections = async () => {
  try {
    console.log('Fetching connections...')
    const response = await fetch(API_BASE_URL + '/get_connections')
    const rawData = await response.json()

    connectionsData.value = rawData

    initGraph()
  } catch (e) {
    error.value = 'Failed to load connections. Please try again later.'
    console.error('Error fetching connections:', e)
  }
}

onMounted(async () => {
  window.addEventListener('resize', updateDimensions)
  updateDimensions()
  await fetchNames()
  await fetchConnections()
})

onUnmounted(() => {
  window.removeEventListener('resize', updateDimensions)
  if (simulation.value) {
    simulation.value.stop()
  }
})
</script>

<template>
  <div class="graph-container">
    <div v-if="error" class="error">{{ error }}</div>
    <svg id="graph"></svg>
  </div>
</template>

<style scoped>
.graph-container {
  width: 100%;
  height: 80vh;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
  overflow: hidden;
}

.error {
  color: #e53935;
  text-align: center;
  padding: 16px;
}

svg {
  display: block;
  margin: 0 auto;
}

/* Style the nodes and links */
:deep(circle) {
  cursor: pointer;
  transition: fill 0.2s;
}

:deep(circle:hover) {
  fill: #ff7f50;
}

:deep(line) {
  stroke-linecap: round;
}

:deep(text) {
  user-select: none;
  pointer-events: none;
}
</style>