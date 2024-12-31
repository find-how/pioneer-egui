import * as EguiAPI from "./egui_api.ts";

class EguiComponent {
  constructor(public id: string) {}
}

class EguiBuilder {
  private components: EguiComponent[] = [];

  addWindow(title: string): WindowBuilder {
    const windowBuilder = new WindowBuilder(title);
    this.components.push(windowBuilder);
    return windowBuilder;
  }

  add3DScene(id: string): Scene3DBuilder {
    return new Scene3DBuilder(id);
  }

  async build(): Promise<void> {
    console.log("UI build is complete");
  }
}

class WindowBuilder extends EguiComponent {
  constructor(title: string) {
    super("window");
    this.initialize(title);
  }

  private initialize(title: string) {
    EguiAPI.addWindow(title).catch(console.error);
  }

  addLabel(id: string): LabelBuilder {
    return new LabelBuilder(id, this);
  }

  addButton(id: string, label: string): ButtonBuilder {
    return new ButtonBuilder(id, label, this);
  }

  addSlider(id: string, range: [number, number]): SliderBuilder {
    return new SliderBuilder(id, range, this);
  }

  addInput(id: string): InputBuilder {
    return new InputBuilder(id, this);
  }

  addCheckbox(id: string): CheckboxBuilder {
    return new CheckboxBuilder(id, this);
  }

  addComboBox(id: string, options: string[]): ComboBoxBuilder {
    return new ComboBoxBuilder(id, options, this);
  }

  addRadioGroup(id: string, options: string[]): RadioGroupBuilder {
    return new RadioGroupBuilder(id, options, this);
  }

  addProgressBar(id: string): ProgressBarBuilder {
    return new ProgressBarBuilder(id, this);
  }

  add3DScene(id: string): Scene3DBuilder {
    return new Scene3DBuilder(id);
  }
}

class LabelBuilder extends EguiComponent {
  constructor(id: string, private builder: WindowBuilder) {
    super(id);
  }

  setText(text: string): WindowBuilder {
    EguiAPI.setLabel(text).catch(console.error);
    return this.builder;
  }
}

class ButtonBuilder extends EguiComponent {
  constructor(id: string, private label: string, private builder: WindowBuilder) {
    super(id);
    this.initialize();
  }

  private initialize() {
    EguiAPI.setButton(this.id, this.label).catch(console.error);
  }

  onClick(handler: () => void): WindowBuilder {
    EguiAPI.onEvent("button_click", () => {
      handler();
    });
    return this.builder;
  }
}

class SliderBuilder extends EguiComponent {
  constructor(id: string, private range: [number, number], private builder: WindowBuilder) {
    super(id);
    this.initialize();
  }

  private initialize() {
    EguiAPI.setSlider(this.range[0]).catch(console.error);
  }

  setValue(value: number): SliderBuilder {
    EguiAPI.setSlider(value).catch(console.error);
    return this;
  }

  onChange(handler: (value: number) => void): WindowBuilder {
    EguiAPI.onEvent("slider_change", (data: { value: number }) => {
      handler(data.value);
    });
    return this.builder;
  }
}

class InputBuilder extends EguiComponent {
  constructor(id: string, private builder: WindowBuilder) {
    super(id);
  }

  setText(text: string): InputBuilder {
    EguiAPI.setInput(text).catch(console.error);
    return this;
  }

  onInput(handler: (text: string) => void): WindowBuilder {
    EguiAPI.onEvent("input_change", (data: { text: string }) => {
      handler(data.text);
    });
    return this.builder;
  }
}

class CheckboxBuilder extends EguiComponent {
  constructor(id: string, private builder: WindowBuilder) {
    super(id);
  }

  setChecked(checked: boolean): CheckboxBuilder {
    EguiAPI.setCheckbox(this.id, checked).catch(console.error);
    return this;
  }

  onToggle(handler: (checked: boolean) => void): WindowBuilder {
    EguiAPI.onEvent(`checkbox_${this.id}`, (data: { checked: boolean }) => {
      handler(data.checked);
    });
    return this.builder;
  }
}

class ComboBoxBuilder extends EguiComponent {
  constructor(id: string, private options: string[], private builder: WindowBuilder) {
    super(id);
    this.initialize();
  }

  private initialize() {
    EguiAPI.setComboBox(this.id, this.options[0], this.options).catch(console.error);
  }

  setSelected(selected: string): ComboBoxBuilder {
    EguiAPI.setComboBox(this.id, selected, this.options).catch(console.error);
    return this;
  }

  onChange(handler: (selected: string) => void): WindowBuilder {
    EguiAPI.onEvent(`combo_${this.id}`, (data: { selected: string }) => {
      handler(data.selected);
    });
    return this.builder;
  }
}

class RadioGroupBuilder extends EguiComponent {
  constructor(id: string, private options: string[], private builder: WindowBuilder) {
    super(id);
    this.initialize();
  }

  private initialize() {
    EguiAPI.setRadio(this.id, this.options[0]).catch(console.error);
  }

  setSelected(selected: string): RadioGroupBuilder {
    EguiAPI.setRadio(this.id, selected).catch(console.error);
    return this;
  }

  onChange(handler: (selected: string) => void): WindowBuilder {
    EguiAPI.onEvent(`radio_${this.id}`, (data: { selected: string }) => {
      handler(data.selected);
    });
    return this.builder;
  }
}

class ProgressBarBuilder extends EguiComponent {
  constructor(id: string, private builder: WindowBuilder) {
    super(id);
  }

  setProgress(value: number): ProgressBarBuilder {
    EguiAPI.setProgress(this.id, value).catch(console.error);
    return this;
  }

  onUpdate(handler: (value: number) => void): WindowBuilder {
    EguiAPI.onEvent(`progress_${this.id}`, (data: { value: number }) => {
      handler(data.value);
    });
    return this.builder;
  }
}

class Scene3DBuilder extends EguiComponent {
  constructor(id: string) {
    super(id);
  }

  addCube(objectId: string, size: number): Scene3DBuilder {
    EguiAPI.add3DObject(this.id, objectId, "cube", size).catch(console.error);
    return this;
  }

  addSphere(objectId: string, radius: number): Scene3DBuilder {
    EguiAPI.add3DObject(this.id, objectId, "sphere", radius).catch(console.error);
    return this;
  }

  rotate(angle: number): Scene3DBuilder {
    EguiAPI.rotate3D(angle).catch(console.error);
    return this;
  }

  onRotate(handler: (angle: number) => void): Scene3DBuilder {
    EguiAPI.onEvent("rotate_3d", (data: { angle: number }) => {
      handler(data.angle);
    });
    return this;
  }

  startRecording(): Scene3DBuilder {
    EguiAPI.startRecording().catch(console.error);
    return this;
  }

  async stopRecording(): Promise<any[]> {
    return await EguiAPI.stopRecording();
  }

  startPlayback(): Scene3DBuilder {
    EguiAPI.startPlayback().catch(console.error);
    return this;
  }

  stopPlayback(): Scene3DBuilder {
    EguiAPI.stopPlayback().catch(console.error);
    return this;
  }
}

// Export the fluent interface
export const pioneer = {
  egui: () => new EguiBuilder(),
};
