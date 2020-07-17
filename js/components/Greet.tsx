import React, { useEffect, useRef } from 'react';
import { useEngines } from '../stores/use_engines';


export function Greet() {
    const canvasRef = useRef<HTMLCanvasElement>(null)
    const { wasmEngine } = useEngines()

    useEffect(() => {
        // Render each frame to a canvas element for Rust to see
        if (wasmEngine.instance && canvasRef.current) {
            wasmEngine.instance.new_game(canvasRef.current)
        }
    }, [canvasRef, wasmEngine])

    return (
        <div>
            <canvas ref={canvasRef} width="640" height="640"></canvas>
        </div>
    );
}