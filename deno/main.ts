import { pioneer } from "./pioneer_egui.ts";
import * as EguiAPI from "./egui_api.ts";

function buildUI() {
  const window = pioneer.egui()
    .addWindow("Timeline Dashboard");

  window
    .addLabel("welcomeLabel")
    .setText("Welcome to Pioneer eGUI with Timeline!")
    .addButton("recordButton", "Start Recording")
    .onClick(() => {
      console.log("Start Recording clicked.");
      pioneer.egui().add3DScene("mainScene").startRecording();
    })
    .addButton("stopRecordButton", "Stop Recording")
    .onClick(async () => {
      console.log("Stop Recording clicked.");
      const recorded = await pioneer.egui().add3DScene("mainScene").stopRecording();
      console.log("Recorded events:", recorded);
      // Let's save them
      await EguiAPI.saveRecordedEvents({ filename: "timeline.json" });
      console.log("Saved to timeline.json");
    })
    .addButton("loadButton", "Load Recording")
    .onClick(async () => {
      console.log("Load Recording clicked.");
      await EguiAPI.loadRecordedEvents({ filename: "timeline.json" });
      console.log("Events loaded from timeline.json");
    })
    .addButton("playbackButton", "Start Playback")
    .onClick(() => {
      console.log("Start Playback clicked.");
      pioneer.egui().add3DScene("mainScene").startPlayback();
    })
    .addButton("stopPlaybackButton", "Stop Playback")
    .onClick(() => {
      console.log("Stop Playback clicked.");
      pioneer.egui().add3DScene("mainScene").stopPlayback();
    });

  window
    .addSlider("volumeSlider", [0, 100])
    .setValue(50)
    .onChange((value) => {
      console.log(`Slider value changed to ${value}`);
    });

  window
    .addInput("usernameInput")
    .setText("John Doe")
    .onInput((text) => {
      console.log(`Input text changed to "${text}"`);
    });

  window
    .addCheckbox("notificationsCheckbox")
    .setChecked(true)
    .onToggle((checked) => {
      console.log(`Checkbox toggled to ${checked}`);
    });

  window
    .addComboBox("themeCombo", ["Light", "Dark", "System"])
    .setSelected("Dark")
    .onChange((selected) => {
      console.log(`ComboBox selected option: ${selected}`);
    });

  window
    .addRadioGroup("languageRadio", ["English", "Spanish", "French"])
    .setSelected("English")
    .onChange((selected) => {
      console.log(`Radio group selected: ${selected}`);
    });

  window
    .addProgressBar("uploadProgress")
    .setProgress(0)
    .onUpdate((value) => {
      console.log(`Progress bar updated to ${value}%`);
    });

  const scene = pioneer.egui()
    .add3DScene("mainScene")
    .addCube("cube1", 1.0)
    .addSphere("sphere1", 0.5)
    .rotate(45)
    .onRotate((angle) => {
      console.log(`3D scene rotated by ${angle} degrees`);
    });

  // Periodically rotate the 3D scene
  setInterval(() => {
    pioneer.egui().add3DScene("mainScene").rotate(15);
  }, 5000);
}

buildUI();

// Keep Deno alive
await new Promise(() => {});
