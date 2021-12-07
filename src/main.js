import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import vuetify from './plugins/vuetify'
import { initializeApp } from "firebase/app";
import { getFirestore } from "firebase/firestore"
// import { getStorage, ref } from "firebase/storage";

Vue.config.productionTip = false

const firebaseConfig = {
  apiKey: "AIzaSyB3dlYeros168ueVSSmjv11NXm6HBYZmoc",
  authDomain: "sharing-text.firebaseapp.com",
  databaseURL: "https://sharing-text-default-rtdb.firebaseio.com",
  projectId: "sharing-text",
  storageBucket: "sharing-text.appspot.com",
  messagingSenderId: "229785112427",
  appId: "1:229785112427:web:50e91aa31d39ab31ab462e"
};

initializeApp(firebaseConfig);

// export const storage = getStorage();
// export const storageRef = ref(storage);
export const db = getFirestore();


new Vue({
  router,
  store,
  vuetify,
  render: h => h(App)
}).$mount('#app')
