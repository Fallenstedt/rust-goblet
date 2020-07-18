import React, { useEffect, useRef } from 'react';
import { useEngines } from '../stores/use_engines';

const styles = {
    container: {
        display: 'flex',
        justifyContent: 'center',
        margin: '0 auto'
    },
    canvas: {
        border: '2px solid red'
    }
}

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
        <div style={styles.container}>
            <canvas style={styles.canvas} ref={canvasRef} width="600" height="800"></canvas>
        </div>
    );
}