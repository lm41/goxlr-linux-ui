<template>
  <Gate />
  <Equiliser />
  <Compressor />
  <DeEsser />
</template>

<script>
import Gate from "./mic/Gate";
import Equiliser from "./mic/Equalizer";
import Compressor from "./mic/Compressor";
import DeEsser from "./mic/DeEsser";
export default {
  name: "Mic",
  components: {DeEsser, Compressor, Equiliser, Gate}
}
</script>

<style scoped>

</style>