import { Reversi } from "gameoflife";
import { memory } from "gameoflife/gameoflife_bg";

const CELL_SIZE = 32; // px
const PIECE_RADIUS = 15; 
const HEIGHT = 8;
const WIDTH = 8;
const N = 64;
const BOARD_COLOR = "#008080";
const WHITE = "#FFFFFF";
const BLACK = "#000000";
let TURN = 0;
let isfirst = true;

const reversi = Reversi.new();

const canvas = document.getElementById("board");
canvas.height = (CELL_SIZE + 1) * HEIGHT + 1;
canvas.width = (CELL_SIZE + 1) * WIDTH + 1;
const ctx = canvas.getContext('2d');
/*
const renderLoop = () => {
  drawGrid();
  drawCells();
  requestAnimationFrame(renderLoop);
};
*/
const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = BLACK;

  // Vertical lines.
  for (let i = 0; i <= WIDTH; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= HEIGHT; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const drawCells = () => {

  ctx.beginPath();
  ctx.fillStyle = BOARD_COLOR
  for (let row = 0; row < HEIGHT; row++) {
    for (let col = 0; col < WIDTH; col++) {
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

const drawPiece = (y, x) => {
    ctx.beginPath();
    ctx.arc(x * (CELL_SIZE + 1) + 2 + PIECE_RADIUS, y * (CELL_SIZE + 1) + 2 + PIECE_RADIUS, PIECE_RADIUS, 0, Math.PI * 2);
    ctx.fill();
}

const playerColor = document.getElementById("playercolor");
playerColor.textContent = "Black";
const countPiece = document.getElementById("count");
countPiece.textContent = "0";
const evaluationValue = document.getElementById("eval");
evaluationValue.textContent = "0";

const initBoard = () => {
    drawGrid();
    drawCells();
    ctx.fillStyle = WHITE;
    drawPiece(3, 3);
    drawPiece(4, 4);
    ctx.fillStyle = BLACK;
    drawPiece(3, 4);
    drawPiece(4, 3);
}


const aiTurn = () => {
  if(isfirst) {
    ctx.fillStyle = WHITE;
  }
  else {
    ctx.fillStyle = BLACK;
  }
  while(true){
    const opflipnum = reversi.search_next_piece();
    if (opflipnum == 0) {
      break;
    }

    const opflipPtr = reversi.flip_list();
    const opflips = new Uint8Array(memory.buffer, opflipPtr, opflipnum);
    for (let pos = 0; pos < opflipnum; pos++) {
      drawPiece(Math.floor(opflips[pos] / WIDTH), opflips[pos] % WIDTH);
    }
    countPiece.textContent = reversi.piece_count().toString();

    evaluationValue.textContent = reversi.eval_value().toString();

    ++TURN;
    if (TURN == N) {
      break;
    }
    if(reversi.is_movable()){
      break;
    }
  }
}


const clearButton = document.getElementById("init_board");

clearButton.textContent = "Clear";

clearButton.addEventListener("click", event => {
  TURN = 0;
  initBoard();
  reversi.clear();
  countPiece.textContent = "0";
  evaluationValue.textContent = reversi.eval_value().toString();
  if (!isfirst) {
    aiTurn();
  }
});

const changeButton = document.getElementById("change");

changeButton.textContent = "Change";

changeButton.addEventListener("click", event => {
  isfirst = !isfirst;
  playerColor.textContent = isfirst? "Black": "White";
  reversi.change();
  aiTurn();
});

const myTurn = (row, col) => {
  const flipnum = reversi.set_op_piece(row * WIDTH + col);
    if(flipnum == 0) {
      return false;
    }
    if(isfirst) {
      ctx.fillStyle = BLACK;
    }
    else {
      ctx.fillStyle = WHITE;
    }
    //drawPiece(row, col);
    const flipPtr = reversi.flip_list();
    const flips = new Uint8Array(memory.buffer, flipPtr, flipnum);
    for (let pos = 0; pos < flipnum; pos++) {
      drawPiece(Math.floor(flips[pos] / WIDTH), flips[pos] % WIDTH);
    }
    countPiece.textContent = reversi.piece_count().toString();
    ++TURN;
    if (TURN == N) {
      return false;
    }
    return true;
}

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();
  
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;
  
    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;
  
    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), HEIGHT - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), WIDTH - 1);
  
    let ok = myTurn(row, col);
    if(ok == false) return;
    setTimeout(aiTurn, 0);
    //aiTurn();
});

initBoard();
/*
drawGrid();
drawCells();
*/