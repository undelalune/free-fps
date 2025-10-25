import {darkTheme, GlobalThemeOverrides, lightTheme} from 'naive-ui'

const darkThemeOverrides: GlobalThemeOverrides = {
    common: {
        ...darkTheme.common,
        primaryColor: '#FF9B2F',
        successColor: '#2fb2ff',
        bodyColor: '#121821',
        textColor2: '#eeeeee',
        // inputColor: '#662549'
        primaryColorHover: '#FF9B2F',
        primaryColorPressed: '#FF9B2F',
        primaryColorSuppl: '#FF9B2F',
        tableColor: '#1a222f',
        tableHeaderColor: '#232e40',
        tableColorHover: '#333333',
        tableColorStriped: '#eeeeee',
        errorColor: '#ff472f',
        invertedColor: '#2fb2ff',
        // actionColor: '#4379F2',
    },
    Button: {
        // ...darkTheme.Button,
        // textColor: '#880000',
        // colorTertiary: '#007bff',
    },
    Tooltip: {
        // ...darkTheme.Tooltip,
        textColor: '#eeeeee',
        color: '#0c0e10',
        boxShadow: 'none',
        padding: '4px 8px',
    },
    FloatButton: {
        // ...darkTheme.FloatButton,
        boxShadow: 'none',
    },
    Progress: {
        fillColor: '#2fb2ff',
        // colorFocus: '#000099',
    },
    InternalSelectMenu: {
        // optionTextColor: '#eeee00',
        // optionTextColorPressed: '#eeee00',
        // optionTextColorDisabled: '#555555',
        optionTextColorActive: '#D1F8EF',
    },
    Switch: {
        // ...darkTheme.Select,
        railColorActive: '#FF9B2F',
        iconColor: '#444444',
    },
    Checkbox: {
        colorChecked: '#FF9B2F'
    },
    Card: {
        borderRadius: '6px',
        colorModal: '#1a222f'
    },
    Slider: {
        indicatorTextColor: '#FF9B2F',
    },
    Message: {
        maxWidth: '400px',
        colorWarning: '#0c0e10',
        iconColorWarning: '#ff9B2F',
    }
}

const lightThemeOverrides: GlobalThemeOverrides = {
    common: {
        ...lightTheme.common,
        bodyColor: '#f3f3f3',
        primaryColor: '#3674B5',
        textColor2: '#222222',
        // textColor2: '#eeeeee',
        // inputColor: '#662549'
        primaryColorHover: '#3674B5',
        primaryColorPressed: '#3674B5',
        primaryColorSuppl: '#3674B5',
        tableColor: '#e8e8e8',
        tableHeaderColor: '#d8d8d8',
        tableColorHover: '#c2c2c2',
        tableColorStriped: '#000000',
        invertedColor: '#2585b9',
        // bodyColor: '#ffffff',
        // textColorBase: '#222222',
        // primaryColor: '#409eff'
    },

    Button: {
        // ...darkTheme.Button,
        // textColor: '#880000',
        // colorTertiary: '#007bff',
    },
    Tooltip: {
        // ...darkTheme.Tooltip,
        textColor: '#eeeeee',
        color: '#444444',
        boxShadow: 'none',
        padding: '4px 8px',
    },
    FloatButton: {
        // ...darkTheme.FloatButton,
        boxShadow: 'none',
    },
    Input: {
        // colorFocus: '#000099',
        border: '1px solid #bcbcbc',
        iconColor: '#aaaaaa',
        placeholderColor: '#888888',
    },
    InternalSelection: {
        border: '1px solid #bcbcbc',
        arrowColor: '#888888',
    },
    InternalSelectMenu: {
        // optionTextColor: '#eeee00',
        // optionTextColorPressed: '#eeee00',
        // optionTextColorDisabled: '#555555',
        optionTextColorActive: '#3674B5',
    },
    Switch: {
        // ...darkTheme.Select,
        railColorActive: '#3674B5',
    },
    Card: {
        borderRadius: '6px',
        colorModal: '#f2f2f2'
    },
    Checkbox: {
        border: '1px solid #bcbcbc',
    },
    Message: {
        maxWidth: '400px',
        colorWarning: '#eeeeee',
        iconColorWarning: '#3674B5',
        textColor: '#222222',
    }
}

export {darkThemeOverrides, lightThemeOverrides}