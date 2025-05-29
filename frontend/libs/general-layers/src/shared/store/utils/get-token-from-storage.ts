export const getTokenFromStorage = (tokenName: string, store: string) => {
    const auth = JSON.parse(localStorage.getItem(`persist:${store}`) ?? '');
    return auth[tokenName].substring(1, auth[tokenName].length - 1);
};
