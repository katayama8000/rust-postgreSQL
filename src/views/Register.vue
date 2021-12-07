<template>
  <v-card width="400px" class="mx-auto mt-5">
    <v-card-title>
      <h1 class="display-1">新規登録</h1>
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
        <v-btn class="info" @click="submit">登録</v-btn>
      </v-form>
    </v-card-text>
  </v-card>
</template>

<script>
//import { getAuth, createUserWithEmailAndPassword } from "firebase/auth";

export default {
  name: "register",
  data() {
    return {
      showPassword: false,
      password: "",
      email:""
    };
  },
  methods: {
    submit: function () {
       this.$store.dispatch("register",{email:this.email, password:this.password })
      // const auth = getAuth();
      // createUserWithEmailAndPassword(auth, this.email, this.password)
      //   .then((userCredential) => {
      //     // Signed in
      //     const user = userCredential.user;
      //     console.log(user);
      //     console.log("やったああああ")
      //   })
      //   .catch((error) => {
      //     const errorCode = error.code;
      //     alert(errorCode);
      //     const errorMessage = error.message;
      //     console.log(errorMessage);
      //   });
    },
  },
};
</script>

