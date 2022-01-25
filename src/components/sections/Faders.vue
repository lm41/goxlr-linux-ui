<template>
  <ContentBox title="Faders">
    <ButtonList title="Channel">
      <Button label="Channel 1" buttonId="0" :is-active="isActiveChannel(0)" @button-pressed="channelPressed" />
      <Button label="Channel 2" buttonId="1" :is-active="isActiveChannel(1)" @button-pressed="channelPressed" />
      <Button label="Channel 3" buttonId="2" :is-active="isActiveChannel(2)" @button-pressed="channelPressed" />
      <Button label="Channel 4" buttonId="3" :is-active="isActiveChannel(3)" @button-pressed="channelPressed" />
    </ButtonList>

    <ButtonList title="Source">
      <Button label="Mic" buttonId="0" :is-active="isActiveSource(0)" @button-pressed="sourcePressed"/>
      <Button label="Voice Chat" buttonId="1" :is-active="isActiveSource(1)" @button-pressed="sourcePressed"/>
      <Button label="Music" buttonId="2" :is-active="isActiveSource(2)" @button-pressed="sourcePressed"/>
      <Button label="Game" buttonId="3" :is-active="isActiveSource(3)" @button-pressed="sourcePressed"/>
      <Button label="Console" buttonId="4" :is-active="isActiveSource(4)" @button-pressed="sourcePressed"/>
      <Button label="Line In" buttonId="5" :is-active="isActiveSource(5)" @button-pressed="sourcePressed"/>
      <Button label="System" buttonId="6" :is-active="isActiveSource(6)" @button-pressed="sourcePressed"/>
      <Button label="Sample" buttonId="7" :is-active="isActiveSource(7)" @button-pressed="sourcePressed"/>
      <Button label="Headphone" buttonId="8" :is-active="isActiveSource(8)" @button-pressed="sourcePressed"/>
      <Button label="Line Out" buttonId="9" :is-active="isActiveSource(9)" @button-pressed="sourcePressed"/>
    </ButtonList>

    <ButtonList title="Mute Behaviour" class="hidden">
      <Button label="Mute All" buttonId="0" :is-active="isActiveMuteBehaviour(0)" :is-disabled="isMuteBehaviourDisabled(0)" @button-pressed="micBehaviourPressed" />
      <Button label="Mute to Stream" buttonId="1" :is-active="isActiveMuteBehaviour(1)" :is-disabled="isMuteBehaviourDisabled(1)" @button-pressed="micBehaviourPressed"/>
      <Button label="Mute to Voice Chat" buttonId="2" :is-active="isActiveMuteBehaviour(2)" :is-disabled="isMuteBehaviourDisabled(2)" @button-pressed="micBehaviourPressed"/>
      <Button label="Mute to Phones" buttonId="3" :is-active="isActiveMuteBehaviour(3)" :is-disabled="isMuteBehaviourDisabled(3)" @button-pressed="micBehaviourPressed"/>
      <Button label="Mute to Line Out" buttonId="4" :is-active="isActiveMuteBehaviour(4)" :is-disabled="isMuteBehaviourDisabled(4)" @button-pressed="micBehaviourPressed"/>
    </ButtonList>
  </ContentBox>
</template>

<script>
import ContentBox from "../ContentBox";
import ButtonList from "../button_list/ButtonList";
import Button from "../button_list/Button";
import {invoke} from "@tauri-apps/api/tauri";

export default {
  /**
   * Everything here focuses around the 'Channel' input,
   */

  components: {ContentBox, ButtonList, Button},
  name: "Faders",

  data() {
    return {
      activeChannel: 0,
      channel: [
        {
          sourceSelection: 0,
          micBehaviour: 0
        },
        {
          sourceSelection: 0,
          micBehaviour: 0
        },
        {
          sourceSelection: 0,
          micBehaviour: 0
        },
        {
          sourceSelection: 0,
          micBehaviour: 0
        },
      ]
    }
  },

  methods: {
    channelPressed: function(id) {
      this.activeChannel = parseInt(id); // parseInt required because javascript :D
    },

    sourcePressed: function(id) {
      this.channel[this.activeChannel].sourceSelection = parseInt(id);

      // If this option is disabled, set to 'All'..
      if (this.isMuteBehaviourDisabled(this.channel[this.activeChannel].micBehaviour)) {
        this.channel[this.activeChannel].micBehaviour = 0;
      }

      // Let Rust know!
      invoke('set_fader_channel', { id: this.activeChannel, channelId: parseInt(id) });
    },

    micBehaviourPressed: function(id) {
      this.channel[this.activeChannel].micBehaviour = parseInt(id);
    },

    isActiveChannel: function (id) {
      if (this.activeChannel === id) {
        return true;
      }
      return false;
    },

    isActiveSource: function (id) {
      if (this.channel[this.activeChannel].sourceSelection === id) {
        return true;
      }
      return false;
    },

    isActiveMuteBehaviour: function(id) {
      if (this.channel[this.activeChannel].micBehaviour === id) {
        return true;
      }
      return false;
    },

    isMuteBehaviourDisabled: function(id) {
      // According to the GoXLR UI, The Voice Chat mute button can't mute to voice chat..
      if (this.channel[this.activeChannel].sourceSelection === 1) {
        if (id === 2) {
          return true;
        }
      }

      // The Headphone and Line Out channels can only mute to 'All'.
      if (this.channel[this.activeChannel].sourceSelection === 8) {
        if (id > 0) {
          return true;
        }
      }
      if (this.channel[this.activeChannel].sourceSelection === 9) {
        if (id > 0) {
          return true;
        }
      }
    },

    getChannelIdByName(name) {
      switch (name) {
        case "Mic": return 0;
        case "LineIn": return 5;
        case "Console": return 4;
        case "System": return 6;
        case "Game": return 3;
        case "Chat": return 1;
        case "Sample": return 7;
        case "Music": return 2;
        case "Headphones": return 8;
        case "LineOut": return 9;
      }
    }
  },

  created: function() {
    // We need to configure the data here from this.$device, so lets go?
    this.channel[0].sourceSelection = this.getChannelIdByName(this.$device.mixer.fader_a_assignment);
    this.channel[1].sourceSelection = this.getChannelIdByName(this.$device.mixer.fader_b_assignment);
    this.channel[2].sourceSelection = this.getChannelIdByName(this.$device.mixer.fader_c_assignment);
    this.channel[3].sourceSelection = this.getChannelIdByName(this.$device.mixer.fader_d_assignment);
  }
}
</script>

<style scoped>
.hidden {
  display: none;
}
</style>