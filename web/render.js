import { Tile } from '../pkg/simulacra.js'

const CELL_SIZE = 5 // px
const COLORS = {
  [Tile.Air]: '#fff',
  [Tile.Water]: '#00f',
  [Tile.Rock]: '#000',
}

function initCtx(width, height) {
  const canvas = document.getElementsByTagName('canvas')[0]
  canvas.width = width * CELL_SIZE
  canvas.height = height * CELL_SIZE
  return canvas.getContext('2d')
}

function drawTiles(ctx, width, height, tiles) {
  ctx.beginPath()
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const index = row * width + col
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
  const width = universe.width()
  const height = universe.height()
  const ctx = initCtx(width, height)

  function renderSingleFrame() {
    const tiles = new Uint8Array(wasm.memory.buffer, universe.tiles_ptr(), width * height)
    drawTiles(ctx, width, height, tiles)
  }

  function render() {
    renderSingleFrame()
    window.requestAnimationFrame(render)
  }

  renderSingleFrame()
  render()
}
