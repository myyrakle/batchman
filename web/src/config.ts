interface Config {
  apiBaseUrl: string;
  isDevelopment: boolean;
  isProduction: boolean;
}

const isDevelopment = process.env.NODE_ENV === 'development';
const isProduction = process.env.NODE_ENV === 'production';

const config: Config = {
  apiBaseUrl: isDevelopment 
    ? 'http://localhost:13939/api'  // 개발 환경 API URL
    : '/api',   // 프로덕션 환경 API URL
  isDevelopment,
  isProduction,
};

export default config; 