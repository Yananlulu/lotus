export default {
  "proxy": {
    "/api": {
      "target": "http://localhost:8080/"
    }
  },
  plugins: [
    ['umi-plugin-dva', {
      immer: true
    }],
  ]
};