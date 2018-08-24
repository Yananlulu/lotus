export const USER_SIGN_IN = 'user.sign-in'
export const USER_SIGN_OUT = 'user.sign-out'

export const userSignIn = (token) => ({type: USER_SIGN_IN, token})
export const userSignOut = () => ({type: USER_SIGN_OUT})
