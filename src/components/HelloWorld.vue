<template>

<div id="main">
  <Tabs>
<!--    <Tab name="Mic">-->
<!--      <div class="contentPad">-->
<!--        <div class="section">-->
<!--          <Mic />-->
<!--        </div>-->
<!--      </div>-->
<!--    </Tab>-->
    <Tab name="Mixer" :selected="true">
      <div class="contentPad">
          <div class="section">
            <Faders />
            <Mixer />
        </div>
      </div>
    </Tab>
    <Tab name="Router" :left_align="false">
      <div class="contentPad">
        <div class="section">
          <Routing />
        </div>
      </div>
    </Tab>
  </Tabs>
</div>

</template>

<script>

import Faders from "./sections/Faders"
import Mixer from "./sections/Mixer";
import Tabs from "@/components/tabs/Tabs";
import Tab from "@/components/tabs/Tab";
import Routing from "@/components/sections/Routing";

export default {
  name: 'HelloWorld',
  components: {Routing, Tab, Tabs, Mixer, Faders},
}
</script>

<style scoped>
#main {
  width: 100%;
  font-size: 10pt;
}

.contentPad {
  display: inline-block;
  padding: 40px;
  text-align: center;
}

.section {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
}

.hidden {
  display: none;
}
</style>
