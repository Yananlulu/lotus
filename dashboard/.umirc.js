export default {
  "proxy": {
    "/api": {
      "target": "http://localhost:8080/"
    }
  },
  base: '/my/',
  hash: true,
  plugins: [
    ['umi-plugin-react', {
      dva: {
        immer: true
      },
      dynamicImport: {
        webpackChunkName: true
      },
      locale: {
        enable: true,
        default: 'en-US',
        baseNavigator: false,
      },
      fastClick: true,
    }],
  ]
};