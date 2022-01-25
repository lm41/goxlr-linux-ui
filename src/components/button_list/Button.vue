<template>
  <div class="button" v-bind:class="{ active: isActive, disabled: isDisabled }" @click="setActive">{{ label }}</div>
</template>

<script>
export default {
  name: "Button",

  props: {
    buttonId: String,
    label: String,
    isActive: Boolean,
    isDisabled: Boolean,
  },

  methods: {
    setActive() {
      console.log("Triggered..");
      // Don't emit for a disabled button.
      if (!this.isDisabled) {
        this.$emit('button-pressed', this.buttonId);
      }
    }
  },
}
</script>

<style scoped>
.button {
  box-sizing: border-box;
  width: calc(100% - 16px);
  margin: 8px;
  background-color: #3b413f;
  padding: 8px;
  text-align: left;
  color: #fff;

  cursor: default;
}

.button:first-child {
  margin-top: 0px;
}

.button:last-child {
  margin-bottom: 0px;
}

.active {
  background-color: #59b1b6;
  color: #353937;
}

.disabled {
  background-color: #383D3B;
  color: #959796;
}
</style>