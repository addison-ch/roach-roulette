import React, { useState, useReducer, useRef } from "react";
import {
  motion,
  animate,
  AnimatePresence,
  useMotionValue,
} from "framer-motion";
import "../styles/GameBoard.css";
// import { DndContext } from "@dnd-kit/core";
// import { Draggable, Droppable, DragDropContext } from "react-beautiful-dnd";
// import { StrictModeDroppable } from "./Droppable";
import { DndContext } from "@dnd-kit/core";

import { Droppable } from "./Droppable";
import { Draggable } from "./Draggable";

const GameBoard: React.FC = () => {
  const [selectedCard, setSelectedCard] = useState(10);
  const list = ["🕷", "🦇", "🐸", "🐀", "🐜", "🐜", "🐜", "🐜"];

  const handleCardClick = (index: number) => {
    setSelectedCard(index);
    console.log(index);
  };

  const handleBlur = () => {
    setSelectedCard(-1);
    console.log("BLURRED");
  };

  //drag kit
  const [cards, setCards] = useState<string[]>([
    "🕷",
    "🦇",
    "🐸",
    "🐀",
    "🐜",
    "🐜",
    "🐜",
    "🐜",
  ]);
  const [yourCardMat, setYourCardMat] = useState<string[]>([]);

  function handleDragEnd(event: any) {
    const { over } = event;
    if (!over) return;
    if (over.id == "A") {
      setYourCardMat([...yourCardMat, cards[selectedCard]]);
      cards.splice(selectedCard, 1);
      setCards(cards);
    }
  }

  return (
    <>
      <DndContext onDragEnd={handleDragEnd}>
        <div style={{ position: "absolute", top: 0 }}>
          <Droppable key="A" id="A">
            {yourCardMat
              .sort((a, b) => a.localeCompare(b))
              .map((card, index) => {
                return (
                  <Draggable
                    key={"yourCardMat" + index}
                    id={"yourCardMat" + index}
                    disabled={true}
                    style={{ cursor: "default" }}
                  >
                    {card}
                  </Draggable>
                );
              })}
          </Droppable>
        </div>
        <div>
          <Droppable key="D" id="D" style={{ height: "200px", width: "800px" }}>
            {cards.map((card, index) => {
              let zValue: any;
              if (index > selectedCard) {
                zValue = list.length - index;
                console.log("VALUE", index, zValue);
              } else if (index < selectedCard) {
                zValue = index;
                console.log("NOT VALUE", index);
              }
              return (
                <Draggable key={"yourHand" + index} id={"yourHand" + index}>
                  <div
                    style={{
                      display: "flex",
                      zIndex: `${zValue}`,
                    }}
                  >
                    <motion.div
                      style={{
                        marginLeft: "-80px",
                        border: `${
                          index === selectedCard ? "1px solid black" : "none"
                        }`,
                      }}
                      onClick={() => handleCardClick(index)}
                      onMouseEnter={() => handleCardClick(index)}
                      whileHover={{ scale: 1.1, zIndex: list.length }}
                      className="card"
                      // tabIndex={0}
                      onBlur={handleBlur}
                    >
                      {card}
                    </motion.div>
                  </div>
                </Draggable>
              );
            })}
          </Droppable>
        </div>
      </DndContext>
    </>
  );
};

export default GameBoard;
