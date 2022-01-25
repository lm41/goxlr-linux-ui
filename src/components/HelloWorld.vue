<template>

<div id="main">
  <div class="section">
    <Faders />
    <Mixer />
  </div>
  <div class="section">
    <Mic />
  </div>
</div>

</template>

<script>

import Faders from "./sections/Faders"
import Mixer from "./sections/Mixer";
import Mic from "./sections/Mic";

export default {
  name: 'HelloWorld',
  components: {Mic, Mixer, Faders},

  methods: {
    getRandom() {
      return Math.floor(Math.random() * 256);
    }
  }
}
</script>

<style scoped>
#main {


  width: 100%;

  font-size: 10pt;
}

.section {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  width: 100%;
}
</style>
