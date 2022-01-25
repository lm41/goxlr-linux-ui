<template>
  <!-- Lets just draw the table.. -->
  <ContentBox title="Routing">
    <table>
      <thead>
        <tr>
          <th colspan="2" class="hidden"></th>
          <th :colspan="inputs.length">Inputs</th>
        </tr>
        <tr class="subHeader">
          <th colspan="2" class="hidden"></th>
          <th v-for="input in inputs" :key="input">{{ input }}</th>
        </tr>
      </thead>

      <tr v-for="output in outputs" :key="output">
        <th v-if="getIndexForOutput(output) === 0" class="rotated" :rowspan="outputs.length"><span>Outputs</span></th>
        <th>{{output}}</th>
        <Cell v-for="input in inputs" :key="input" :enabled="isEnabled(getIndexForOutput(output), getIndexForInput(input))" :output="getIndexForOutput(output)" :input="getIndexForInput(input)" @clicked="handleClick"/>
      </tr>

      <tr><td colspan="10" class="hidden" style="height: 10px"></td></tr>
      <tr>
        <!-- Sampler is a little weird, it's on a line by itself because reasons? -->
        <th colspan="2">Sampler</th>
        <Cell v-for="input in inputs" :key="input" :enabled="isEnabled(-1, getIndexForInput(input))" :output="-1" :input="getIndexForInput(input)" @clicked="handleClick" />
      </tr>
    </table>
  </ContentBox>
</template>

<script>
import ContentBox from "@/components/ContentBox";
import Cell from "@/components/sections/routing/Cell";
export default {
  name: "Routing",
  components: {Cell, ContentBox},

  data() {
    return {
      inputs: ["Mic", "Chat", "Music", "Game", "Console", "Line In", "System", "Samples"],
      outputs: ["Headphones", "Broadcast Mix", "Line Out", "Chat Mic"],

      state: [],
    }
  },

  computed: {
    inputCount() {
      return this.inputs.length;
    }
  },

  created() {
    // Generate a 'base' map..
    for (let i = -1; i < this.outputs.length; i++) {
      this.state[i] = [];
      for (let j = 0; j < this.inputs.length; j++) {
        this.state[i][j] = false;
      }
    }
  },

  methods: {
    handleClick: function(output, input) {
      this.state[output][input] = !this.state[output][input];
    },

    isEnabled: function(output, input) {
      return this.state[output][input];
    },

    getIndexForOutput(name) {
      for (let i = 0; i < this.outputs.length; i++) {
        if (name === this.outputs[i]) {
          return i;
        }
      }
    },

    getIndexForInput(name) {
      for (let i = 0; i < this.inputs.length; i++) {
        if (name === this.inputs[i]) {
          return i;
        }
      }
    }
  }
}
</script>

<style scoped>

table {
  color: #fff;
  font-stretch: condensed;
  border-spacing: 4px;
  border-collapse: separate;
}

th {
  font-weight: normal;
  padding: 6px;
}

thead th:not(.subHeader) {
  background-color: #3b413f;
}

thead .subHeader th {
  background-color: #353937;
  width: 70px;
}

tr th {
  background-color: #353937;
}


.rotated {
  background-color: #3b413f;
  text-align: center;
}

.rotated span {
  writing-mode: vertical-rl;
  transform: rotate(180deg);
  white-space: nowrap;
}

.hidden {
  background-color: transparent !important;
}
</style>