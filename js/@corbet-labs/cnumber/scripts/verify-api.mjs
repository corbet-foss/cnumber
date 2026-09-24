// This module also runs in a real browser with no Node shims.
export function verify(api) {
    const check = (actual, expected) => {
        if (JSON.stringify(actual) !== JSON.stringify(expected)) {
            throw new Error(`${JSON.stringify(actual)} !== ${JSON.stringify(expected)}`);
        }
    };
    check(api.formatNumber('de-ch', '1234567.89', true, 'amtlich'), '1\'234\'567,89');
    check(api.formatMoney('de-ch', '1234.56', 'CHF', 'symbol'), 'Fr. 1\'234.56');
    check(api.formatMoney('de-ch', '100000', 'CHF', 'code'), 'CHF 100\'000');
    check(api.formatMoney('de', '1234567.89', 'EUR', 'code'), 'EUR 1.234.567,89');
    check(api.formatMoney('de-ch', '1234.56', 'CHF', 'short'), null);
    check(api.formatNumber('de', '12,34'), null);
}
