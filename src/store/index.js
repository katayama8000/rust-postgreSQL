import Vue from 'vue'
import Vuex from 'vuex'
import router from "../router/index"
import {getAuth}  from "firebase/auth";
import { signInWithEmailAndPassword } from "firebase/auth";
import { createUserWithEmailAndPassword } from "firebase/auth";

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    idToken: null,
    detailText:null
  },

  getters:{
    idToken:state => state.idToken
  },

  mutations: {
    upDataIdToken(state, idToken) {
      state.idToken = idToken
    },

    upDataDetailText(state, detailText) {
      state.detailText = detailText
    }
  },

  actions: {
    async login({ commit }, authData) {

      const auth = getAuth();
      signInWithEmailAndPassword(auth, authData.email, authData.password)
        .then((userCredential) => {
          // Signed in
          const user = userCredential.user;
          alert("ログイン成功");
          commit("upDataIdToken",user.uid)

          router.push("/main")
        })
        .catch((error) => {
          const errorCode = error.code;
          alert(errorCode);
        })
      },


    register({ commit }, authData) {
      const auth = getAuth();
      createUserWithEmailAndPassword(auth, authData.email, authData.password)
        .then((userCredential) => {
          // Signed in
          const user = userCredential.user;
          alert("新規登録完了", user);
          commit("upDataIdToken",user.uid)

          router.push("/main")

        })
        .catch((error) => {
          const errorCode = error.code;
          alert("エラー\n新しいメールアドレスorパスワードを\n入力してください",errorCode);
        })
      },

      logout({commit}){
        commit("upDataIdToken",null)
        //localStorage.removeItem
        router.replace("/login")
      }
    },
  modules: {
  }
})
