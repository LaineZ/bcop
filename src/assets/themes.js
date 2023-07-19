let currentTheme = "hope_diamond"

const themes = {
    hope_diamond: {
        "bg": "#150e10",
        "bg1": "#272739",
        "bg1_gradient": "#272739C8",
        "bg2": "#393849",
        "fg": "#77888c",
        "fg2": "#4f5a64",
        "fg3": "#3b4152",
    },

    nord: {
        "bg": "#2e3440",
        "bg1": "#3b4252",
        "bg1_gradient": "#2e3440C8",
        "bg2": "#434c5e",
        "fg": "#eceff4",
        "fg2": "#d8dee9",
        "fg3": "#4c566a",
    },

    rust: {
        "bg": "#230000",
        "bg1": "#712f30",
        "bg1_gradient": "#712f30C8",
        "bg2": "#a54932",
        "fg": "#ffe2c6",
        "fg2": "#f0bb9c",
        "fg3": "#e18866",
    },

    rainy_day: {
        "bg": "#1D3557",
        "bg1": "#31587A",
        "bg1_gradient": "#31587AC8",
        "bg2": "#457B9D",
        "fg": "#F1FAEE",
        "fg2": "#A8DADC",
        "fg3": "#77ABBD",
    },

    infinity: {
        "bg": "#F5F5EB",
        "bg1": "#CACACA",
        "bg1_gradient": "#CACACAC8",
        "bg2": "#969696",
        "fg": "#1F232D",
        "fg2": "#424242",
        "fg3": "#696969",
    },

    molten: {
        "bg": "#201727",
        "bg1": "#261b2e",
        "bg1_gradient": "#261b2eC8",
        "bg2": "#382d43",
        "fg": "#fd724e",
        "fg2": "#a02f40",
        "fg3": "#5f2f45",
    },

    lush_green: {
        "bg": "#39425c",
        "bg1": "#3c4c63",
        "bg1_gradient": "#3c4c63C8",
        "bg2": "#3f5b69",
        "fg": "#469c58",
        "fg2": "#3a875e",
        "fg3": "#417070",
    }
}

function setTheme(theme) {
    currentTheme = theme;
    let root = document.documentElement;
    if (themes[theme]) {
        for (const [key, value] of Object.entries(themes[theme])) {
            root.style.setProperty("--" + key, value);
        }
    } else {
        logWarn("Unable to find theme: " + theme + " reverting to hope_diamond...");
        setTheme("hope_diamond");
    }
}

