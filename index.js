import { Universe } from './pkg/simulacra.js'
import { hookRender } from './render.js'

const universe = Universe.new(100, 100)
const button = document.getElementById('tick')
button.addEventListener('click', function() {
  universe.tick()
})

export default async function(wasm) {
  hookRender(universe, wasm)
}
