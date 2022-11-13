#!/bin/sh

# Variables
DIRECTORY=~/temporary/

ZFS_POOL_NAME=zfs_pool
ENCRYPTED_POOL_NAME=encrypted_pool
ARCHIVE_POOL_NAME=archive_pool

ZFS_POOL_FILE=${DIRECTORY}${ZFS_POOL_NAME}
ENCRYPTED_POOL_FILE=${DIRECTORY}${ENCRYPTED_POOL_NAME}
ARCHIVE_POOL_FILE=${DIRECTORY}${ARCHIVE_POOL_NAME}

# Processes

if [ $# -ne 1 ]; then
    echo "Specify the argument of 'create' or 'destroy.'"
    exit 1
fi

if [ ${1} = 'create' ]; then

    echo "Creating files..."
    mkdir -p ${DIRECTORY}

    dd if=/dev/zero of=${ZFS_POOL_FILE} bs=64M count=1
    dd if=/dev/zero of=${ENCRYPTED_POOL_FILE} bs=64M count=1
    dd if=/dev/zero of=${ARCHIVE_POOL_FILE} bs=64M count=1

    echo "Creating ZFS pools..."
    sudo zpool create ${ZFS_POOL_NAME} `readlink -f ${ZFS_POOL_FILE}`
    sudo zpool create ${ENCRYPTED_POOL_NAME} `readlink -f ${ENCRYPTED_POOL_FILE}`
    sudo zpool create ${ARCHIVE_POOL_NAME} `readlink -f ${ARCHIVE_POOL_FILE}`

elif [ ${1} = 'destroy' ]; then

    echo "Destroy ZFS pools..."
    sudo zpool destroy ${ZFS_POOL_NAME}
    sudo zpool destroy ${ENCRYPTED_POOL_NAME}
    sudo zpool destroy ${ARCHIVE_POOL_NAME}

    rm -r ${DIRECTORY}
else
    echo "Specify 'create' or 'destroy' as the argument."
    exit 1
fi
