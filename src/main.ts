import {createApp} from 'vue'
import './style.css'
import App from './App.vue'

// 通用字体
import 'vfonts/Lato.css'
// 等宽字体
import 'vfonts/FiraCode.css'
import naive from "naive-ui";

const app = createApp(App)
app.use(naive)
app.mount('#app')

