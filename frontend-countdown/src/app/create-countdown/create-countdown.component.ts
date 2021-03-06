import { Component, OnInit, ElementRef } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import backendURL from '../backendURL';
import { Router } from '@angular/router';
import { HandleErrorsService } from '../error-handler.service';

@Component({
  selector: 'app-create-countdown',
  templateUrl: './create-countdown.component.html',
  styleUrls: ['./create-countdown.component.css']
})
export class CreateCountdownComponent implements OnInit {

  theme: boolean = (document.body.getAttribute('data-theme') == "light" ? true : false);
  title: string = "";
  year: string = "2020";
  month: string = "12";
  day: string = "31";
  hour: string = "16";
  minute: string = "30";

  constructor(private http: HttpClient, public router: Router, public errService: HandleErrorsService, private el: ElementRef) { }

  ngOnInit() {
    var currentDate = new Date();
    this.year = currentDate.getFullYear().toString();
    this.month = (this.padZeros(currentDate.getMonth() + 1, 2)).toString();
    this.day = this.padZeros(currentDate.getDate(), 2).toString();
    this.hour = this.padZeros(currentDate.getHours(), 2).toString();
    this.minute = this.padZeros(currentDate.getMinutes(), 2).toString();
  }

  padZeros(data, desiredLength) {
    while (data.toString().length < desiredLength) {
      data = "0" + data.toString();
    }
    return data;
  }

  create() {
    var timeToSet = new Date(parseInt(this.year), parseInt(this.month) - 1, parseInt(this.day), parseInt(this.hour), parseInt(this.minute), 0, 0);
    var utcString = timeToSet.toUTCString();

    this.http.post<{
      success: boolean;
      errors: [];
      id: string;
      name: string;
      datetime: string;
    }>(backendURL + "/create",
    {
      "name": {"data": this.title, "id": "input-title"},
      "datetime": {"data": utcString, "id": "input-date"}
    })
    .subscribe(
      data  => {
        console.log("POST Request is successful ", data);
        if (data.success) {
          this.router.navigate(['/id/' + data.id]);
        } else {
          this.errService.handleErrors(this.el, data.errors, ["input-title", "input-date", "input-time"]);
        }
      },
      error  => {
        console.log("Error", error);
        this.errService.handleErrors(this.el, [{"id": "fatal", "reason": "Unable to perform request. Please try again."}], []);
      }

    );
  }

  themeChange() {
    if (this.theme) {
      document.body.setAttribute('data-theme', 'light')
    } else {
      document.body.setAttribute('data-theme', 'dark')
    }
  }

}
