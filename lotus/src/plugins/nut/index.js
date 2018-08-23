export default {
  menus: [],
  routes: [{
    component: () =>
      import ('./Home'),
    path: '/'
  }, {
    component: () =>
      import ('./Install'),
    path: '/install'
  }, {
    component: () =>
      import ('./users/SignIn'),
    path: '/users/sign-in'
  }, {
    component: () =>
      import ('./users/SignUp'),
    path: '/users/sign-up'
  }]
}