module.exports = {
    content: [
        '*.html',
        './src/**/*.rs',
    ],
    theme: {
        colors: {
            'background': {
                DEFAULT: '#f7fff7',
                100: '#006400',
                200: '#00c800',
                300: '#2dff2d',
                400: '#91ff91',
                500: '#f7fff7',
                600: '#f7fff7',
                700: '#f9fff9',
                800: '#fbfffb',
                900: '#fdfffd'
            },
            'text': {
                DEFAULT: '#343434',
                100: '#0a0a0a',
                200: '#141414',
                300: '#1f1f1f',
                400: '#292929',
                500: '#343434',
                600: '#5c5c5c',
                700: '#858585',
                800: '#adadad',
                900: '#d6d6d6'
            },
            'primary': {
                DEFAULT: '#1244E6',
                100: '#0c0c19',
                200: '#181832',
                300: '#24244a',
                400: '#303163',
                500: '#3c3d7c',
                600: '#5152a8',
                700: '#7b7cbf',
                800: '#a7a8d5',
                900: '#d3d3ea'
            },
            'secondary': {
                DEFAULT: '#228252',
            }
        },
        fontFamily: {
            display: ['Inter Tight', 'sans-serif'],
            sans: ['Inter', 'sans-serif']
        },
        extend: {
            flex: {
                '2': '2 2 0%'
            }
        }
    },
    safelist: [
        'hidden',
    ],
}
