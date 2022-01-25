<template>
  <ContentBox title="Mixer">
    <Slider title="MIC" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
    <Slider title="CHAT" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
    <Slider title="MUSIC" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
    <Slider title="GAME" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
    <Slider title="CONSOLE" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
    <Slider title="LINE IN" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
    <Slider title="LINE out" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
    <Slider title="SYSTEM" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
    <Slider title="SAMPLE" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
    <div style="margin-right: 30px"></div>
    <Slider title="BLEEP" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
  </ContentBox>
  <ContentBox title="Headphones">
    <Slider title="LEVEL" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value="10" />
    <div :class="{ hidden: !isVisible }">
      <Slider title="MIC MONITOR" :slider-min-value=0 :slider-max-value=255 :text-min-value=0 :text-max-value=100 text-suffix="%" :slider-value=10 />
    </div>
  </ContentBox>
  <ExpandoBox @expando-clicked="toggleExpando" :expanded="isVisible" />
</template>

<script>
import Slider from "../slider/Slider";
import ContentBox from "../ContentBox";
import ExpandoBox from "../expandoBox/ExpandoBox";

export default {
  name: "Mixer",
  components: {ExpandoBox, ContentBox, Slider},

  data() {
    return {
      isVisible: false,
    }
  },

  methods: {
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