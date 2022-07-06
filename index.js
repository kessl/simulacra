import { Universe } from './pkg/simulacra.js'

const universe = Universe.new()
const pre = document.getElementById('dump')
const button = document.getElementById('tick')
button.addEventListener('click', function() {
  universe.tick()
})

export default async function(module) {
  function render() {
    const cells = new Uint32Array(module.memory.buffer, universe.cells_ptr(), universe.cells_count());
    pre.innerHTML = cells
    window.requestAnimationFrame(render)
  }
  render()
}
