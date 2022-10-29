
module.exports = {
    content: [ "./src/**/*.rs", "./index.html" ],

    theme: {
        extend: {
            fontFamily: {
                sans: ['Nunito', 'sans'],
            },
        },
    },

    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
        require('daisyui')
    ],
    daisyui: {
        themes: ["lofi", "black"],
    },
};
