// Copy next to a fresh installation to test the actual npm tarball.
import { createRequire } from 'node:module';
import { verify } from './verify-api.mjs';
verify(await import('@corbet-labs/cnumber'));
verify(createRequire(import.meta.url)('@corbet-labs/cnumber'));
verify(await import('@corbet-labs/cnumber/browser'));
console.log('cnumber: installed ESM, CommonJS, and browser exports passed');
