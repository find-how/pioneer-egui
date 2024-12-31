// Simple event emitter implementation
class SimpleEventEmitter {
  private listeners: Map<string, Array<(data: any) => void>> = new Map();

  on(event: string, handler: (data: any) => void): void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event)?.push(handler);
  }

  emit(event: string, data: any): void {
    this.listeners.get(event)?.forEach(handler => handler(data));
  }
}

const eventEmitter = new SimpleEventEmitter();

/**
 * Connect to Rust's WebSocket server and forward messages to the local event emitter.
 */
async function connectToWebSocket() {
  try {
    const ws = new WebSocket("ws://127.0.0.1:9001");

    ws.onopen = () => {
      console.log("Connected to Rust WebSocket server");
      // Send a test message
      ws.send(JSON.stringify({ type: "hello", message: "Hello from Deno!" }));
    };

    ws.onmessage = (event) => {
      const data = event.data;
      console.log("Received event from Rust:", data);
      try {
        const parsed = JSON.parse(data);
        eventEmitter.emit(parsed.type, parsed);
      } catch (e) {
        console.log("Received non-JSON message:", data);
      }
    };

    ws.onclose = () => {
      console.log("WebSocket connection closed");
      // Try to reconnect after a delay
      setTimeout(() => connectToWebSocket(), 5000);
    };

    ws.onerror = (error) => {
      console.error("WebSocket error:", error);
    };

    return ws;
  } catch (error) {
    console.error("Failed to connect to WebSocket:", error);
    // Try to reconnect after a delay
    setTimeout(() => connectToWebSocket(), 5000);
    return null;
  }
}

// Start the WebSocket connection
let ws = connectToWebSocket();

// Listen to an event
export function onEvent(event: string, handler: (data: any) => void): void {
  eventEmitter.on(event, handler);
}

// Mock implementations for development
async function mockOpAsync(name: string, args: any): Promise<any> {
  console.log(`Mock ${name} called with:`, args);
  return Promise.resolve();
}

export async function addWindow(title: string): Promise<void> {
  await mockOpAsync("op_set_label", { text: `Window titled "${title}"` });
  console.log(`Simulated window creation: "${title}"`);
}

export async function setLabel(text: string): Promise<void> {
  await mockOpAsync("op_set_label", { text });
}

export async function setButton(id: string, label: string): Promise<void> {
  console.log(`Set button: ${id} with label: ${label} (rust side is a no-op unless extended)`);
}

export async function setSlider(value: number): Promise<void> {
  await mockOpAsync("op_set_slider", { value });
}

export async function setInput(text: string): Promise<void> {
  await mockOpAsync("op_set_input", { text });
}

export async function setCheckbox(id: string, checked: boolean): Promise<void> {
  await mockOpAsync("op_set_checkbox", { id, checked });
}

export async function setComboBox(id: string, selected: string, options: string[]): Promise<void> {
  await mockOpAsync("op_set_combo_box", { id, selected, options });
}

export async function setRadio(id: string, selected: string): Promise<void> {
  await mockOpAsync("op_set_radio", { id, selected });
}

export async function setProgress(id: string, value: number): Promise<void> {
  await mockOpAsync("op_set_progress", { id, value });
}

export async function rotate3D(angle: number): Promise<void> {
  await mockOpAsync("op_rotate_3d", { angle });
}

export async function add3DObject(sceneId: string, objectId: string, objectType: string, size: number): Promise<void> {
  await mockOpAsync("op_add_3d_object", {
    scene_id: sceneId,
    object_id: objectId,
    object_type: objectType,
    size,
  });
}

export async function startRecording(): Promise<void> {
  await mockOpAsync("op_start_recording", {});
}

export async function stopRecording(): Promise<any[]> {
  return await mockOpAsync("op_stop_recording", {});
}

export async function startPlayback(): Promise<void> {
  await mockOpAsync("op_start_playback", {});
}

export async function stopPlayback(): Promise<void> {
  await mockOpAsync("op_stop_playback", {});
}

export async function saveRecordedEvents(args: { filename: string }): Promise<void> {
  await mockOpAsync("op_save_recorded_events", args);
}

export async function loadRecordedEvents(args: { filename: string }): Promise<void> {
  await mockOpAsync("op_load_recorded_events", args);
}
