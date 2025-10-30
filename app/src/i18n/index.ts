import {createI18n} from "vue-i18n";
import en from './locales/en';
import pl from './locales/pl';
import by from './locales/by';
import ru from './locales/ru';
import ua from './locales/ua';
import de from './locales/de';
import fr from './locales/fr';
import es from './locales/es';
import pt from './locales/pt';
import it from './locales/it';

export type Locale = 'en' | 'pl' | 'ru' | 'ua' | 'by' | 'de' | 'fr' | 'es' | 'pt' | 'it';
export type UILanguage = {
    value: Locale;
    label: string;
}
export const locales = {
    en: en,
    pl: pl,
    by: by,
    ru: ru,
    ua: ua,
    de: de,
    fr: fr,
    es: es,
    pt: pt,
    it: it,
};


export const languages: UILanguage[] = [
    {value: 'by', label: 'Беларуская'},
    {value: 'de', label: 'Deutsch'},
    {value: 'en', label: 'English'},
    {value: 'es', label: 'Español'},
    {value: 'fr', label: 'Français'},
    {value: 'it', label: 'Italiano'},
    {value: 'pl', label: 'Polski'},
    {value: 'pt', label: 'Português'},
    {value: 'ru', label: 'Русский'},
    {value: 'ua', label: 'Українська'},
];

const instance = createI18n({
    locale: 'en',
    fallbackLocale: 'en',
    globalInjection: true,
    messages: locales,
    legacy: false,
})

export default instance;

export const i18n = instance.global;
