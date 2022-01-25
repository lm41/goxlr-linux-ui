<template>
  <ContentBox title="Mixer">
    <Slider :id=1 title="MIC" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(0) @value-changed="valueChange" />
    <Slider :id=2 title="CHAT" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(5) @value-changed="valueChange" />
    <Slider :id=3 title="MUSIC" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(7) @value-changed="valueChange" />
    <Slider :id=4 title="GAME" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(4) @value-changed="valueChange" />
    <Slider :id=5 title="CONSOLE" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(2) @value-changed="valueChange" />
    <Slider :id=6 title="LINE IN" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(1) @value-changed="valueChange" />
    <Slider :id=7 title="LINE OUT" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(10) @value-changed="valueChange" />
    <Slider :id=8 title="SYSTEM" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(3) @value-changed="valueChange" />
    <Slider :id=9 title="SAMPLE" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(6) @value-changed="valueChange" />
    <div style="margin-right: 30px"></div>
    <Slider title="BLEEP" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
  </ContentBox>
  <ContentBox title="Headphones">
    <Slider :id=10 title="LEVEL" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(8) @value-changed="valueChange" />
    <div :class="{ hidden: !isVisible }">
      <Slider :id=11 title="MIC MONITOR" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=getValue(9) @value-changed="valueChange" />
    </div>
  </ContentBox>
  <ExpandoBox @expando-clicked="toggleExpando" :expanded="isVisible" />
</template>

<script>
import Slider from "../slider/Slider";
import ContentBox from "../ContentBox";
import ExpandoBox from "../expandoBox/ExpandoBox";
import { invoke } from "@tauri-apps/api/tauri";

export default {
  name: "Mixer",
  components: {ExpandoBox, ContentBox, Slider},

  data() {
    return {
      isVisible: false,
    }
  },

  methods: {
    valueChange(id, newValue) {
      // Let rust know..
      invoke('set_channel_volume', { id: id, volume: newValue })
    },

    getValue(id) {
      return this.$device.mixer.volumes[id];
    },

    hideExpanded() {
      return false;
    },

    toggleExpando() {
      this.isVisible = !this.isVisible;
    }
  }
}
</script>

<style scoped>
.hidden {
  visibility: hidden;
  display: none;
}
</style>