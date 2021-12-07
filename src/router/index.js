import Vue from 'vue'
import VueRouter from 'vue-router'
import store from "../store/index"
import Home from '../views/Home.vue'
import Top from '../views/Top.vue'
import Login from '../views/Login.vue'
import Register from '../views/Register.vue'
import Main from '../views/Main.vue'
import Detail from '../views/Detail.vue'
import Sell from '../views/Sell.vue'
import PersonalInfo from '../views/InputInfo.vue'
import Profile from '../views/Profile.vue'
import Chat from '../views/Chat.vue'
import Todo from "../views/Todo"
import Time from "../views/Time"
import Google from "../views/GoogleLogin.vue"

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/top',
    name: 'Top',
    component: Top
  },
  {
    path: '/todo',
    name: 'Todo',
    component: Todo
  },
  {
    path: '/login',
    name: 'Login',
    component: Login,
     //tokenがあったらメインへ
    beforeEnter(to,from,next){
      if(store.getters.idToken){
        next("/main")
      }else{
        next()
      }
    }
  },
  {
    path: '/register',
    name: 'Register',
    component: Register,
    //tokenがあったらメインへ
    beforeEnter(to,from,next){
      if(store.getters.idToken){
        next("/main")
      }else{
        next()
      }
    }
  },
  {
    path: '/main',
    name: 'Main',
    component: Main,
     //tokenがなかったらloginへ
    // beforeEnter(to,from,next){
    //   if(store.getters.idToken){
    //     next()
    //   }else{
    //     alert("login or 新規登録をしてください")
    //     next("/login")
    //   }
    // }
  },
  {
    path: '/detail',
    name: 'Detail',
    component: Detail
  },
  {
    path: '/sell',
    name: 'Sell',
    component: Sell
  },
  {
    path: '/personalinfo',
    name: 'PersonalInfo',
    component: PersonalInfo
  },
  {
    path: '/profile',
    name: 'Profile',
    component: Profile
  },
  {
    path: '/chat',
    name: 'Chat',
    component: Chat
  },
  {
    path: '/time',
    name: 'Time',
    component: Time
  },
  {
    path: '/google',
    name: 'Google',
    component: Google
  },
  {
    path: '/about',
    name: 'About',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/About.vue')
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

export default router
