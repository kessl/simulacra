const CELL_SIZE = 5 // px
const GRID_COLOR = '#ddd'

function initCtx(width, height) {
  const canvas = document.getElementsByTagName('canvas')[0]
  canvas.width = width * (CELL_SIZE + 1) + 1
  canvas.height = height * (CELL_SIZE + 1) + 1
  return canvas.getContext('2d')
}

function drawGrid(ctx, width, height) {
  ctx.beginPath()
  ctx.strokeStyle = GRID_COLOR

  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0)
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1)
  }

  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1)
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1)
  }

  ctx.stroke()
}

function drawCells(ctx, width, height, cells) {
  ctx.beginPath()
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const index = row * width + col
      ctx.fillStyle = cells[index] % 7 ? '#fff' : '#000'
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
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
    const cells = new Uint32Array(wasm.memory.buffer, universe.cells_ptr(), width * height)
    drawGrid(ctx, width, height)
    drawCells(ctx, width, height, cells)
  }

  function render() {
    renderSingleFrame()
    window.requestAnimationFrame(render)
  }

  renderSingleFrame()
  render()
}
