// Copy next to a fresh installation to test the actual npm tarball.
import { createRequire } from 'node:module';
import { verify } from './verify-api.mjs';
verify(await import('@corbet-foss/cnumber'));
verify(createRequire(import.meta.url)('@corbet-foss/cnumber'));
verify(await import('@corbet-foss/cnumber/browser'));
console.log('cnumber: installed ESM, CommonJS, and browser exports passed');
