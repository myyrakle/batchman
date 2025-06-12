interface Config {
  apiBaseUrl: string;
  isDevelopment: boolean;
  isProduction: boolean;
}

const isDevelopment = process.env.NODE_ENV === 'development';
const isProduction = process.env.NODE_ENV === 'production';

const config: Config = {
  apiBaseUrl: '/api',
  isDevelopment,
  isProduction,
};

export default config; 