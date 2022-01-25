<template>
  <div id="sliderInput">
    <input type="number" v-on:input="update" v-on:blur="reset" v-model="localTextValue" :min="minValue" :max="maxValue" />
  </div>
</template>

<script>
export default {
  name: "Input",

  data() {
    return {
      localTextValue: 0,
      lastTextValue: 0
    }
  },

  props: {
    currentTextValue: Number,
    minValue: { type: Number, default: 0 },
    maxValue: { type: Number, default: 100 },
  },

  methods: {
    update(e) {
      let newValue = e.target.value;

      if (newValue === "-" || newValue === "") {
        // Cleared box, or starting negative value..
        return;
      }

      if (e.target.value > this.maxValue) {
        newValue = this.maxValue;
        this.localTextValue = this.maxValue;
      }

      if (e.target.value < this.minValue) {
        newValue = this.minValue;
        this.localTextValue = this.minValue;
      }

      // Value has changed, emit something upwards..
      this.$emit("value-updated", parseInt(newValue));
    },

    reset(e) {
      e.target.value = this.lastTextValue;
    }
  },

  watch: {
    currentTextValue: function(newValue) {
      this.localTextValue = newValue;
      this.lastTextValue = newValue;
    }
  },
}
</script>

<style scoped>
#sliderInput input[type=number] {
  background-color: #3b413f;
  color: #59b1b6;
  padding: 10px;
  box-sizing: border-box;

  text-align: center;

  width: 100%;

  margin-top: 15px;

  border:none;
  background-image:none;
  box-shadow: none;
  outline: none;
}

#sliderInput  input[type=number]::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>