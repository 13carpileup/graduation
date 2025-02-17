<script setup lang="ts">
import * as d3 from 'd3'
import { ref, onMounted, onUnmounted } from 'vue'
import { API_BASE_URL } from '@/constants'

interface Node {
  id: string | number
  name: string
  x?: number
  y?: number
  fx?: number | null
  fy?: number | null
}

interface Link {
  source: Node
  target: Node
  value: number
}

interface GraphData {
  nodes: Node[]
  links: Link[]
}

const nodes = ref<Node[]>([])
const data = ref<GraphData>({ nodes: [], links: [] })
const error = ref<string>('')
const simulation = ref<any>(null)
const width = ref<number>(800)
const height = ref<number>(1000)

const colourmap = new Map<string, string>()
const idmap = new Map<string | number, string>()
const connectionsData = ref<[number, number, number][][]>([[], [], []])
const buttonClasses = ref<[string, string, string]>(["selected", "unselected", "unselected"])
const dataIndex = ref<number>(0)


const updateDimensions = () => {
  const container = document.querySelector('.graph-container')
  if (container) {
    width.value = container.clientWidth
    height.value = container.clientHeight
  }
}

const fetchNames = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/get_all_names`)
    const data = await response.json()

    for (let i = 0; i < data.length; i++) {
      // console.log(data[i])
      idmap.set(data[i].id, data[i].name)
      nodes.value.push({
        id: data[i].id,
        name: data[i].name,
      })
    }
    //console.log("Names fetched!", nodes.value)
  } catch (e) {
    error.value = 'Failed to load names. Please try again later.'
    console.error('Error fetching names:', e)
  }
}

const drag = (simulation: any) => {
  const dragstarted = (event: any, d: any) => {
    if (!event.active && simulation.value) simulation.value.alphaTarget(0.3).restart()
    d.fx = d.x
    d.fy = d.y
  }

  const dragged = (event: any, d: any) => {
    d.fx = event.x
    d.fy = event.y
  }

  const dragended = (event: any, d: any) => {
    if (!event.active && simulation.value) simulation.value.alphaTarget(0)
    d.fx = null
    d.fy = null
  }

  return d3.drag()
    .on('start', dragstarted)
    .on('drag', dragged)
    .on('end', dragended)
}

const initGraph = () => {
  //console.log('Initiating data...')


  
  const validNodeIds = new Set(nodes.value.map(node => node.id.toString()))
  
  const min = 0
  
  console.log("accessing" + connectionsData.value[0])

  nodes.value.forEach(node => {
    node.x = (node.id as number) % 33;
    node.y = (node.id as number) % 33;
  });

  const links = connectionsData.value[dataIndex.value]
  .filter((conn: number[]) => {
    const sourceExists = validNodeIds.has(conn[0].toString());
    const targetExists = validNodeIds.has(conn[1].toString());

    if (!sourceExists) {
      console.warn(`Source node ${conn[0]} not found`);
    }
    if (!targetExists) {
      console.warn(`Target node ${conn[1]} not found`);
    }

    return sourceExists && targetExists && conn[2] > min;
  })
  .map((conn: any[]) => {
    const sourceNode = nodes.value.find(node => node.id.toString() === conn[0].toString());
    const targetNode = nodes.value.find(node => node.id.toString() === conn[1].toString());

    if (!sourceNode || !targetNode) {
      console.warn('Invalid source or target node:', conn);
      return null; 
    }

    return {
      source: sourceNode,
      target: targetNode,
      value: (2) ** (conn[2] - 4),
      strength: conn[2]
    };
  })
  .filter((link: any) => link !== null) as Link[]; 

  //console.log('Valid links created:', links)

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

  const zoom = d3.zoom<any, any>()
    .scaleExtent([0.08, 6])
    .on('zoom', (event) => {
      g.attr('transform', event.transform)
    })
  
  svg.call(zoom).call(zoom.transform, d3.zoomIdentity.scale(0.7))

  const link = g.append('g')
    .selectAll('line')
    .data(data.value.links)
    .join('line')
    .attr('stroke', '#999')
    .attr('stroke-opacity', 0.6)
    .attr('stroke-width', d => Math.sqrt(d.value) / 4)

  const node = g.append('g')
    .selectAll('g')
    .data(data.value.nodes)
    .join('g')
    .call(drag(simulation) as any)

  node.append('circle')
    .attr('r', 8)
    .attr('fill', d => colourmap.get(d.id as string) || '#69b3a2')

  node.append('text')
    .text(d => d.name)
    .attr('x', 12)
    .attr('y', 4)
    .style('font-size', '12px')
    .style('fill', '#333')



  if (nodes.value.length > 0) {
      simulation.value = d3.forceSimulation(data.value.nodes)
        .force('link', d3.forceLink(data.value.links).strength((link: any) => link.strength / 30)
          .id((d: any) => d.id)
          .distance(60))
      .force('charge', d3.forceManyBody().strength(-300))
      .force('center', d3.forceCenter(width.value / 2, height.value / 2))
      .on('tick', () => {
        link
          .attr('x1', d => (d.source as any).x)
          .attr('y1', d => (d.source as any).y)
          .attr('x2', d => (d.target as any).x)
          .attr('y2', d => (d.target as any).y)

        node
          .attr('transform', d => `translate(${(d as any).x},${(d as any).y})`)
      })
  }

  // nodes.value.forEach(node => {
  //   node.x = 30;
  //   node.y = 20;
  // });
}

const fetchConnections = async () => {
  try {
    console.log('Fetching connections...')
    const response = await fetch(`${API_BASE_URL}/get_connections`)
    const rawData = await response.json()
    console.log(rawData)

    rawData[0][0].forEach((d: [[number, number], number]) => {
      connectionsData.value[0].push([d[0][0], d[0][1], d[1] / 10])
      connectionsData.value[2].push([d[0][0], d[0][1], d[1] / 10])
    });

    rawData[0][1].forEach((d: [[number, number], number]) => {
      connectionsData.value[1].push([d[0][0], d[0][1], d[1] / 10])
      connectionsData.value[2].push([d[0][0], d[0][1], d[1] / 10])
    });

    rawData[1].forEach((d : [{id: string, name: string}, [number, number, number]]) => {
      const hexR = d[1][0].toString(16).padStart(2, '0');
      const hexG = d[1][1].toString(16).padStart(2, '0');
      const hexB = d[1][2].toString(16).padStart(2, '0');
      const fullHex = `#${hexR}${hexG}${hexB}`;

      console.log(d[1], fullHex);
      
      colourmap.set(d[0].id, fullHex);
    })

    //console.log(connectionsData.value);

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

const setChosen = (index: number) => {
  for (let i = 0; i < buttonClasses.value.length; i++) {
    buttonClasses.value[i] = "unselected";
  }

  buttonClasses.value[index] = "selected";

  dataIndex.value = index;

  d3.select('#graph').selectAll('*').remove()

  if (simulation.value) {
    simulation.value.stop()
  }

  initGraph();
}
</script>

<template>

  <div class="graph-container">
    <div v-if="connectionsData[0].length === 0" class="loading">
        Loading connections...
    </div>

    <div v-if="error" class="error">{{ error }}</div>
    <svg id="graph"></svg>
  </div>
  <div class = "option-wrapper" :key="buttonClasses[0]">
    <h3 class = "option-text">Sort by:</h3>
    <button @click = "setChosen(0)" :class = "'set-view ' + buttonClasses[0]">Shared Classes</button>
    <button @click = "setChosen(1)" :class = "'set-view ' + buttonClasses[1]">Subject Combinations</button>
    <button @click = "setChosen(2)" :class = "'set-view ' + buttonClasses[2]">Mix</button>
    <h4 class = "guide-text">Scroll to zoom, drag to pan around</h4>
  </div>  
</template>

<style scoped>
.guide-text {
  font-size: 1rem;
  font-weight: bold;
  background: linear-gradient(to right, #906bd2, #807be3);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  margin-bottom: 1rem;
  margin-left: 10rem;
  font-style: italic;
  float: right;
}

.loading {
  text-align: center;
  margin: 0 auto;
  line-height: 200px;
}

.option-wrapper {
  display:flex;
  flex-direction: row;
  align-items: baseline;
  flex-wrap: wrap;
}

.selected {
  background-color: rgb(219, 219, 255)
}

.unselected {
  background-color: rgb(217, 212, 212);
}

.set-view {
  transition: background-color; 
  transition-duration: 300ms;
  margin: 1rem;
  min-height: 40px;
  border: none;
  border-radius: 5px;
  padding: 10px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
}

.option-text {
  margin-right: 2rem;
}

.set-view.unselected:hover {
  background-color: rgb(201, 201, 233);
}


.option-text {
  font-size: 1.5rem;
  font-weight: bold;
  background: linear-gradient(to right, #7c3aed, #4f46e5);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  margin-bottom: 1rem;
  margin-left: 4rem;
}
.graph-container {
  margin: 0 auto;
  width: 90%;
  height: 80vh;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
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