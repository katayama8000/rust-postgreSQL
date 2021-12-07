<template>
  <v-card width="400px" class="mx-auto mt-5">
    <v-card-title>
      <h1 class="display-1">ログイン</h1>
    </v-card-title>
    <v-card-text>
      <v-form>
        <v-text-field
          prepend-icon="mdi-account-circle"
          label="email"
          v-model="email"
        />
        <v-text-field
          v-bind:type="showPassword ? 'text' : 'password'"
          @click:append="showPassword = !showPassword"
          prepend-icon="mdi-lock"
          v-bind:append-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
          label="パスワード"
          v-model="password"
        />
        <v-row>
        <v-btn class="info" @click="submit">ログイン</v-btn>
        <v-spacer></v-spacer>
        <span>登録していない方はこちら → </span>
        <v-btn><router-link to="/register">新規登録</router-link></v-btn>

        </v-row>
      </v-form>
    </v-card-text>
   
  </v-card>
</template>

<script>
//import { getAuth, signInWithEmailAndPassword } from "firebase/auth";
export default {
  data() {
    return {
      showPassword: false,
      password: "",
      email: "",
    };
  },
  methods: {

    submit() {
      this.$store.dispatch("login",{email:this.email, password:this.password })
      // const auth = getAuth();
      // signInWithEmailAndPassword(auth, this.email, this.password)
      //   .then((userCredential) => {
      //     // Signed in
      //     const user = userCredential.user;
      //     alert("ログイン成功",user);
      //     console.log(user.uid)
      //     //this.$router.push("/main")
      //   })
      //   .catch((error) => {
      //     const errorCode = error.code;
      //     alert(errorCode);
      //     const errorMessage = error.message;
      //     alert(errorMessage);
      //   });
     },
  },
};
</script>