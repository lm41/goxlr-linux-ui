<template>
  <div id="sliderBox">
    <Label v-bind:title="title" />
    <Range :current-field-value=fieldValue :min-value="sliderMinValue" :max-value="sliderMaxValue" @value-updated="sliderValueUpdated" />
    <Input :current-text-value="textValue" :min-value="textMinValue" :max-value="textMaxValue" @value-updated="inputValueUpdated"  />
  </div>
</template>

<script>
/**
 * So, how this works, there are 7 props, but the only properties that are important are related
 * to the 'slider' component as those will be the 'real' values sent to the GoXLR device. The
 * input boxes below them are an 'abstract' representation of the slider state (for example,
 * a percentage) which are all dynamically handled, calculated, and presented based on the general
 * definitions provided.
 *
 * TODO: Some sliders have custom stepping, that can accelerate after certain points, we need to find
 * a way to support similar behaviour..
 */

import Label from "./components/Label";
import Range from "./components/Range";
import Input from "./components/Input";
export default {
  name: "Slider",
  components: {Input, Range, Label},

  data() {
    return {
      fieldValue: 0,
      textValue: 0,
    }
  },

  props: {
    id: { type: Number, default: -1 },

    title: { type: String, default: "UNSET" },

    sliderMinValue: Number,
    sliderMaxValue: Number,
    sliderValue: Number,

    textMinValue: Number,
    textMaxValue: Number,
    textSuffix: String,
  },

  methods: {
    sliderValueUpdated(newValue) {
      this.fieldValue = parseInt(newValue);

      this.calculateTextValue();
    },

    inputValueUpdated(newValue) {
      // Webkit / Blink are interesting in that they wont prevent a user from entering an invalid
      // number, so we use this code to force any entry into our boundries.

      if (newValue > this.textMaxValue) {
        this.textValue = 0;
        this.textValue = this.textMaxValue;
      } else if (newValue < this.textMinValue) {
        this.textValue = this.textMinValue;
      } else {
        this.textValue = newValue;
      }

      this.calculateFieldValue();

    },

    calculateTextValue() {
      // Get the distances between the slider, and text numbers..
      let sliderDistance = this.sliderMaxValue - this.sliderMinValue;
      let textDistance = this.textMaxValue - this.textMinValue;

      // Get the position of the slider in the range..
      let position = this.getPosition(parseInt(this.fieldValue), this.sliderMinValue, this.sliderMaxValue);

      // With the position known, we should be able to calculate the 'position' on the text range..
      let input = Math.round((position / sliderDistance) * textDistance);

      // Now set it..
      this.textValue = this.textMinValue + input;
    },

    calculateFieldValue() {
      // Same as above, although this can probably be abstracted..
      let sliderDistance = this.sliderMaxValue - this.sliderMinValue;
      let textDistance = this.textMaxValue - this.textMinValue;

      let position = this.getPosition(parseInt(this.textValue), this.textMinValue, this.textMaxValue);

      // With the position known, we should be able to calculate the 'position' on the text range..
      let input = Math.round((position / textDistance) * sliderDistance);
      this.fieldValue = this.sliderMinValue + input;
    },

    getPosition(find, start, end) {
      // This really shouldn't exist, but I could find a way to get the position of a value between two numbers
      // without some quick iteration..
      for (let i = start, position = 0; i <= end; i++, position++) {
        if (i === find) {
          return position;
        }
      }
    }

  },

  computed: {
    getValue() {
      return this.fieldValue;
    }
  },

  mounted() {
    this.fieldValue = this.sliderValue;
    this.calculateTextValue();
  },

  watch: {
    fieldValue: function(newValue) {
      this.$emit('value-changed', this.id, newValue);
    }
  }
}
</script>

<style scoped>
  #sliderBox {
    margin: 3px;
    width: 90px;
    background-color: #353937;
  }
</style>