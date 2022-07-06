import { Tile } from '../pkg/simulacra.js'

const CELL_SIZE = 2 // px
const COLORS = {
  [Tile.Air]: '#fff',
  [Tile.Water]: '#00f',
  [Tile.Rock]: '#000',
}

function initCtx(size) {
  const canvas = document.getElementsByTagName('canvas')[0]
  canvas.width = canvas.height = size * CELL_SIZE
  return canvas.getContext('2d')
}

function drawTiles(ctx, size, tiles) {
  ctx.beginPath()
  for (let row = 0; row < size; row++) {
    for (let col = 0; col < size; col++) {
      const index = row * size + col
      ctx.fillStyle = COLORS[tiles[index]]
      ctx.fillRect(
        col * CELL_SIZE,
        row * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE
      )
    }
  }
  ctx.stroke()
}

export function hookRender(universe, wasm) {
  const size = universe.size()
  const ctx = initCtx(size)

  function renderSingleFrame() {
    ctx.clearRect(0, 0, size * CELL_SIZE, size * CELL_SIZE)
    const tiles = new Uint8Array(wasm.memory.buffer, universe.tiles_ptr(), size * size)
    drawTiles(ctx, size, tiles)
  }

  function render() {
    renderSingleFrame()
    window.requestAnimationFrame(render)
  }

  renderSingleFrame()
  render()
}
