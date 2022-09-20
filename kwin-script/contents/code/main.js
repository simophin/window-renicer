workspace.clientActivated.connect(function (client) {
    if (client) {
        callDBus('dev.fanchao.WindowRenicer',
                 '/dev/fanchao/WindowRenicer',
                 'dev.fanchao.WindowRenicer',
                 'WindowActivated',
                 client.pid.toString()
        );
    }
});

workspace.clientRemoved.connect(function (client) {
    if (client) {
        callDBus('dev.fanchao.WindowRenicer',
                 '/dev/fanchao/WindowRenicer',
                 'dev.fanchao.WindowRenicer',
                 'WindowRemoved',
                 client.pid.toString()
        );
    }
});
