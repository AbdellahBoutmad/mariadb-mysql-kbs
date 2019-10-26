'use strict';

const realTypes = [
    'string',
    'boolean',
    'integer',
    'numeric',
    'enumeration',
    'set',
    'directory name',
    'file name',
    'byte',
];

/**
 * Clean type using real types
 * @param {String} type The type
 * @return {String|undefined} The cleaned type
 */
const cleanType = function(type) {
    if (realTypes.includes(type) === false && typeof type === 'string') {
        if (type.match(/in bytes/i) || type.match(/number of bytes/i) || type.match(/size in mb/i)) {
            type = 'byte';
        } else if (
            type.match(/number of/i) ||
            type.match(/size of/i) ||
            type.match(/in microseconds/i) ||
            type.match(/in seconds/i)
        ) {
            type = 'integer';
        } else if (
            type.match(/numeric (64-bit unsigned integer)/i) ||
            type.match(/numeric (32-bit unsigned integer)/i)
        ) {
            type = 'numeric';
        } else {
            type = undefined;
        }
    }
    return type;
};

const regexCli = /([-]{2})([0-9a-z-_]+)/i;

/**
 * Clean cli argument
 * @param {String} cli The command line string
 * @param {boolean} skipRegex Skip regex check
 * @returns {String} The cleaned cli
 */
const cleanCli = function(cli, skipRegex = false) {
    if (typeof cli === 'string') {
        if (cli.match(/<code\>/i) || cli.match(/<\/code\>/i)) {
            cli = cli.replace(/<code\>/gi, '');
            cli = cli.replace(/<\/code\>/gi, '');
            cli = cli.replace(/\>/gi, '');
            cli = cli.replace(/</gi, '');
        }
        if (!cli.match(regexCli) && skipRegex === false) {
            cli = undefined;
        }
    }
    return cli;
};

/**
 * Clean the range object
 * @param {Object} range The range object
 * @returns {Object} The cleaned range object
 */
const cleanRange = function(range) {
    if (range !== undefined) {
        // clean range
        if (typeof range.from !== 'number' || isNaN(range.from)) {
            delete range.from;
        }
        if (typeof range.to === 'string' && range.to.match(/upwards/i)) {
            range.to = 'upwards';
        } else if (typeof range.to !== 'number' || isNaN(range.to)) {
            delete range.to;
        }
    }
    return range;
};

/**
 * Clean a default value
 * @param {String} defaultValue The default value
 */
const cleanDefault = function(defaultValue) {
    if (defaultValue === 'Autosized (see description)') {
        defaultValue = '(autosized)';
    }
    if (defaultValue.indexOf('Based on the number of processors') !== -1) {
        defaultValue = '(based on the number of processors)';
    }
    if (defaultValue === 'The MariaDB data directory') {
        defaultValue = '(the MariaDB data directory)';
    }
    if (defaultValue.match(/-1 \(signifies (autoscaling); do not assign this literal value\)/g)) {
        defaultValue = '(-1 signifies autoscaling; do not use -1)';
    }
    if (defaultValue.match(/-1 \(signifies (autosizing); do not assign this literal value\)/g)) {
        defaultValue = '(-1 signifies autosizing; do not use -1)';
    }
    return defaultValue;
};

module.exports = {
    regexCli: regexCli,
    cleanType: cleanType,
    cleanCli: cleanCli,
    cleanRange: cleanRange,
    cleanDefault: cleanDefault,
};
