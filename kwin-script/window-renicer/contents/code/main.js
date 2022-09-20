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