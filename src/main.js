import {createApp} from 'vue'
import App from './App.vue'
import {invoke} from "@tauri-apps/api/tauri";

let app = createApp(App);

// Load the default GoXLR State prior to loading the window..
invoke('test_command', {param: "Hello From javascript!"})
    .then(function (result) {
        app.config.globalProperties.$device = result.deviceStatus;
        app.config.globalProperties.$device.routingMap = result.routingMap;
        app.mount('#app')
    });
